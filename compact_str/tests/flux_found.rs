//! CENTRIFUGE: repros of what Fluxing found. See `log/BUGS.md`.
use compact_str::CompactString;

/// Says it has more items than there are, which safe code may do.
struct Liar<T>(Vec<T>);

impl<T> Iterator for Liar<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

// Bug 1. In a release build each of these reaches `unreachable_unchecked` instead: build with
// `RUSTFLAGS=-Zub-checks=yes` and `--release` to see it, or run Miri.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "capacity <= MAX_VALUE")]
fn bug1_try_with_capacity() {
    let _ = CompactString::try_with_capacity(1usize << 60);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "capacity <= MAX_VALUE")]
fn bug1_try_reserve() {
    let _ = CompactString::new("abc").try_reserve(usize::MAX - 3);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "capacity <= MAX_VALUE")]
fn bug1_collect_chars() {
    let _: CompactString = Liar(vec!['a']).collect();
}

// Bug 2. `String` collects the same iterator without complaint.
#[test]
fn bug2_string_is_fine() {
    let s: String = Liar(vec!["x".repeat(30), "x".repeat(30)]).collect();
    assert_eq!(s.len(), 60);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn bug2_collect_strings() {
    let _: CompactString = Liar(vec!["x".repeat(30), "x".repeat(30)]).collect();
}
