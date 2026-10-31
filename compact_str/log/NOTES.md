# Harness notes, compact_str

Target: `ParkMyCar/compact_str` `9696af7` (0.10.0), the `compact_str/` directory, imported as a tree
copy. `readme` now points at the crate's own `README.md`. See also `ABORT.md`, `BUGS.md`,
`FLUX_ISSUES.md`.

## Score (2026-10-31)

Driver `da3a598c44`, nightly-2026-10-22, libs `bd5de27`. `cargo flux` is clean on all of these. `checked`
includes statics, so it can exceed `processed - trusted`.

| Build | processed | checked | trusted |
|---|---:|---:|---:|
| macOS, default features | 324 | 270 | 58 |
| Linux x86_64, default features | 324 | 270 | 58 |
| macOS, `bytes,smallvec` | 333 | 278 | 59 |

Trusted attributes in the source: 25 ASSUMED, 17 ABORT (one in a macro that expands 10 times), 2 BUG,
3 ICE and 1 `trusted_impl` ICE. One ICE is `bytes` only.

`cargo test --lib --test flux_found`: 259 + 5 pass, 279 + 5 with `bytes,smallvec`.

Other features weren't run. `CARGO_BUILD_TARGET` is the way to change target. The model is 64-bit
little-endian only.

## Canary audit

Fully checked: `lib.rs` 139 of 139 functions. The remaining silent lines in `repr/*`, `num.rs` and
`unicode_data.rs` were each read and are expected:

- `capacity`, `allocate_ptr`, `deallocate_ptr`, `from_string`, `into_string`, `realloc`: the 32-bit
  `cap.is_heap()` arms. `realloc` also has the `new_size < new_capacity` guard, dead because
  `Layout[capacity, 1]` is exact.
- `num_chars` for `usize`/`isize`, `ensure_read`: `cfg`'d arms.
- `assume`, `new_const`: the branch their precondition rules out.

**Not audited:** `traits.rs` (the canary timed out twice at 570s; it has no unsafe code outside tests but
the `LifetimeFree` marker impls), `macros.rs`, `features/*`, and `repr/bytes.rs` and `repr/smallvec.rs`,
because the canary runs default features.

The canary found one real hole, see `FLUX_ISSUES.md`. `harness/flux_canary.py` needed a fix: it unpacked
3 values from a `run_cargo` that returns 4 on main. socket2 and dlmalloc-rs still carry the broken
version since the rebase.

## The model

`Repr` is indexed by its real fields: `hlen`, the capacity bytes `c0`, `c1`, `c2`, and `last`, the
`LastByte`, which is why that enum has 218 `variant` lines. `opaque` is off limits and flux needs every
`refined_by` parameter to be determined by a field. `Capacity` is indexed by its encoded bits. `rlen`,
`rcap` and `ocap` in `repr/mod.rs` decode them. So `len`, `is_empty`, `capacity` and the discriminant
checks are body-checked against the layout.

What's assumed (25 `ASSUMED`):

- Transmutes and pointer casts between `Repr` and its views.
- `Capacity`'s bit encoding, tested upstream.
- **Inline length.** An array has no index, so `from_inline` takes the length as an extra argument.
  Every call site passes the length it built the buffer with, but flux can't see that.
- **`set_len` at 24 on an inline string** doesn't touch the last byte: it relies on the caller having
  written it. `set_len`'s contract says that.
- `disjoint`: flux has no notion of distinct allocations. Each call is commented and is compiled only
  under `cfg(flux)`.
- `str_len(s) <= 2^56 - 2`, on `str::len` and `String::capacity`. It's upstream's assumption too.

Only memory bounds are specified. UTF-8 validity isn't modelled, so a `Repr` being valid UTF-8 is unproved.

## Source changes that aren't annotations

All behave the same at runtime, and each is marked `CENTRIFUGE`.

- `from_inline` has a `_len` argument. New helpers: `empty`, `as_static_view`, `take_string`,
  `as_inline_ptr`, `owned_ptr`, `make_room`, `heap_realloc`, `extend_chars`, `request_capacity`,
  `hint_capacity`, `owned_spare`, `chunk`, `chunk_mut`, `word_at`.
- `do_alloc`: `match` for `ok_or`. `replace_range`: `if` for `match cmp`. `repeat`: `match` for `expect`.
- `as_mut_ptr` and `push_str` are split at an `if`, because flux loses the index of `self` over the join.
- `ensure_read` has no `asm!` under `cfg(flux)`.

## Traps worth keeping

- An `invariant` on `CompactString` over `rlen`/`rcap` made the solver hang. A discarded
  `self.capacity()` under `cfg(flux)` gives the same fact where needed: 7s for a sweep.
- `usize{v: v == rlen(r)}` beats `usize[rlen(r)]` when a loop needs a bound: the existential puts a
  variable in scope for inference to use.
- `harness/checkpoint.sh` does `git add -A`, which would commit `to-file/` here. Stage by hand.
- `git checkout -b` half-fails in the sandbox, which can't lock `.git/config`: the worktree moves but
  HEAD doesn't. Use `--no-track`.
