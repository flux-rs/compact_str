# Bugs, compact_str

Repros: `tests/flux_found.rs`.

## Bug 1: `Capacity::new` assumes what no caller checks (UB from safe code)

`Capacity::new` calls `assume(capacity.to_le() & !VALID_MASK == 0)`, which is `unreachable_unchecked`
in a release build. Nothing bounds the capacity first. Flux flagged it: with `requires capacity <=
MAX_VALUE`, `allocate_ptr` and `HeapBuffer::realloc` can't discharge it.

- `CompactString::try_with_capacity(1 << 60)`
- `s.try_reserve(usize::MAX - 3)`
- `collect()` or `extend()` of `char`s with a huge `size_hint`. `FromIterator<char>` and `Extend<char>`
  go out of their way to ignore a failed reservation, because `size_hint` can be wrong. It gets here
  first.

Debug builds panic on `capacity <= MAX_VALUE`. Release builds return `Err`, so it looks fine, but
`-Zub-checks=yes` aborts on `hint::unreachable_unchecked must never be reached`.

**Upstream:** already filed as ParkMyCar/compact_str#495 by rhysd, with `try_with_capacity` only. Open
PR #496 turns the `debug_assert!` into an `assert!`, which covers all three routes, though `try_*`
then panics rather than returning `Err`.

Trusted as `heap::request_capacity`, a one-line helper, so the rest of both callers stays checked.

## Bug 2: `size_hint` can overflow the capacity sum (debug panic)

`from_as_ref_str_iterator` computes `bytes_len + curr_len + min_remaining`, where `min_remaining` is the
iterator's `size_hint`. That is unchecked, so a hint near `usize::MAX` panics with `attempt to add with
overflow` in a debug build. `String` collects the same iterator. A release build wraps, gets a wrong
capacity and carries on, so there's no memory unsafety.

**Upstream:** no matching issue found.

Trusted as `repr::iter::hint_capacity`.

## Seen, not found by this work

PR #498 (open, rhysd) says `from_utf8_buf` is unsound: `reserve` makes a `&str` of bytes not yet
validated. That's in `collect_buf` under the `bytes` feature, which is trusted here on an ICE, so flux
says nothing about it.
