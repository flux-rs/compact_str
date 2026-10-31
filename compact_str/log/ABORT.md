# ABORT log, compact_str

Every entry was reached by removing the label and reading flux's error. Line numbers go stale, so
entries are keyed by function. Attribute counts below; `into_repr` is one attribute in a macro that
expands for 10 integer types.

| § | Cause | Attributes | Functions |
|---|---|---:|---|
| 1 | Capacity on the heap exists on 32-bit only | 6 | `read_capacity_from_heap`, `allocate_with_capacity_on_heap`, `deallocate_with_capacity_on_heap`, `heap_capacity::{alloc, dealloc, layout}` |
| 2 | No relation between `Chars` and the string | 5 | `Repr::pop`, `CompactString::{remove, retain}`, `SetLenOnDrop::drop`, `map_uppercase_sigma` |
| 3 | Bounds come from table contents | 1 | `unicode_data::skip_search` |
| 4 | Digit count vs digits written | 1 (10 expansions) | `IntoRepr::into_repr` in `impl_IntoRepr!` |
| 5 | `Utf8Error` isn't tied to its input | 1 | `CompactString::from_utf8_lossy` |
| 6 | `Drain` holds a raw pointer | 1 | `Drain::drop` |
| 7 | `str` slicing has no spec | 1 | `CompactString::split_off` |
| 8 | flux-core's `DerefMut` signature | 1 | `<CompactString as DerefMut>::deref_mut` |

## §1 Heap capacity

`Capacity::is_heap` is `false` on 64-bit and has that as its spec, which is the only target flux runs
on here, so these are unreachable and take `requires false` where a caller needs a spec. Their bodies
have never been checked. The specs of `Capacity` and `cap_bits` are 64-bit little-endian too.

Not tried: a 32-bit run. The model is written for one layout, so it would need a second set of specs.

## §2 `Chars`

```
error: arithmetic operation may overflow      self.len() - ch.len_utf8()
error: a precondition cannot be proved        .expect("cannot remove a char from the end of a string")
```

Nothing says a `char` from `chars()` is part of the string, or that `chars().next()` on a non-empty
string is `Some`.

Tried: a spec on `len_utf8` (1..=4), which clears the overflow only where a bound on the string is
already known. Not tried: modelling `Chars`, which would need an index on it. The safety of `retain` and
`remove` depends on exactly this fact, so they are unverified in flux. `retain`'s guard is included:
`g.self_` is a `&mut`, so it has no index to relate `dst_idx` to.

## §3 Tables

```
error: possible out-of-bounds access          short_offset_runs[last_idx]
error: arithmetic operation may overflow      decode_length(*next) - offset_idx
```

The bounds hold because the tables are sorted and end above `char::MAX`, which is what the upstream
comment says. flux arrays have no element-level refinement. `decode_length` and `decode_prefix_sum`
are checked, the latter through the `ShlOne` property.

## §4 Digits

About 25 errors per integer type (256 over the 10 expansions), at the `offset(curr)` calls and the `curr -= n` lines: nothing relates
`curr`, which starts at `num_chars(self)`, to the value of `n`.

An invariant `n < 10^curr` looks possible for unsigned types, but it needs a table of powers and a
qualifier over it, and signed types go through `(!(x as u64)).wrapping_add(1)`, where `!` has no
meaning in flux. That is not attempted. The `num_chars` bodies are checked.

## §5 `Utf8Error`

```
error: a precondition cannot be proved        &remaining[..valid_up_to]
```

Slicing here is checked at runtime, so the cost is a possible panic, not memory safety.

## §6 `Drain`

`Drain::drop` calls `replace_range_shrink` through `*mut CompactString`, which has no index, and with
the literal `""`, whose `str_len` flux doesn't know.

## §7 `str` slicing

`self[at..]` panics if `at` is out of range, so `set_len(at)` after it is safe. An extern spec on the
blanket `impl<I: SliceIndex<str>> Index<I> for str` needs a `RangeFrom`-only form, which is rejected:
`extern specs must exactly match the external definition`.

## §8 `DerefMut`

```
error: a precondition cannot be proved        fn deref_mut(&mut self) -> &mut str
```

flux-core's trait spec is `fn(self: &mut Self[@v]) -> ..`, so an impl must keep the index of `self`.
This one can't: `as_mut_str` turns a static string into an owned one, which changes the index. An
unannotated impl fails the trait check. A spec that keeps the index is what's trusted, so a caller that
goes through `deref_mut` keeps a stale index on a static string. In this crate that's harmless: the
length doesn't change, and only `rcap` would be out of date.
