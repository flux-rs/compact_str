//! CENTRIFUGE: flux-only refinement-level definitions and extern specs.
//! Nothing here affects compiled code.

use alloc::string::String;
use core::alloc::{Layout, LayoutError};

flux_rs::defs! {
    // Bytes in a `str`. A `str`'s flux index is its value, which says nothing about its length.
    fn str_len(s: str) -> int;

    // `p` can be read or written for `n` bytes.
    fn readable(p: ptr, n: int) -> bool {
        p.addr != 0 && p.base <= p.addr && n <= p.size
    }

    // `n` bytes at `p` and at `q` are valid and don't overlap.
    fn apart(p: ptr, q: ptr, n: int) -> bool {
        p.addr + n <= q.addr || q.addr + n <= p.addr
    }
}

#[flux_rs::extern_spec(core::str)]
impl str {
    // Upstream's own assumption: "it's impossible to create a string 64 petabytes or larger".
    #[flux_rs::spec(fn(&str[@s]) -> usize[str_len(s)] ensures 0 <= str_len(s) && str_len(s) <= 72057594037927934)]
    fn len(&self) -> usize;

    #[flux_rs::spec(fn(&str[@s]) -> bool[str_len(s) == 0])]
    fn is_empty(&self) -> bool;

    #[flux_rs::spec(fn(&str[@s]) -> *const{p: readable(p, str_len(s))} u8)]
    fn as_ptr(&self) -> *const u8;

    #[flux_rs::spec(fn(&str[@s]) -> &[u8][str_len(s)])]
    fn as_bytes(&self) -> &[u8];
}

#[flux_rs::extern_spec(core::str)]
#[flux_rs::spec(fn(&[u8][@n]) -> &str{s: str_len(s) == n})]
unsafe fn from_utf8_unchecked(v: &[u8]) -> &str;

/// Ghost: `dst` is a fresh allocation or a `&mut`, so it can't overlap `src`.
#[flux_rs::trusted(reason = "ASSUMED: flux has no notion of distinct allocations; each call says why")]
#[flux_rs::spec(fn(src: *const[@p] u8, dst: *mut[@q] u8, n: usize) ensures apart(p, q, n))]
pub(crate) fn disjoint(_src: *const u8, _dst: *mut u8, _n: usize) {}

#[flux_rs::extern_spec(core::cmp)]
#[flux_rs::spec(fn(v1: T[@a], v2: T[@b]) -> T{v: <T as Ord>::max_res(a, b, v)})]
fn max<T: Ord>(v1: T, v2: T) -> T;

#[flux_rs::extern_spec(core::cmp)]
#[flux_rs::spec(fn(v1: T[@a], v2: T[@b]) -> T{v: <T as Ord>::min_res(a, b, v)})]
fn min<T: Ord>(v1: T, v2: T) -> T;

#[flux_rs::extern_spec(core::slice)]
#[flux_rs::spec(fn(data: *const[@p] T, len: usize) -> &[T][len]
                requires readable(p, len * T::size_of()))]
unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T];

#[flux_rs::extern_spec(core::slice)]
#[flux_rs::spec(fn(data: *mut[@p] T, len: usize) -> &mut [T][len]
                requires readable(p, len * T::size_of()))]
unsafe fn from_raw_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T];

#[flux_rs::extern_spec(core::alloc)]
impl Layout {
    // Only for a prefix that needs no padding: there the result is exact.
    #[flux_rs::spec(fn(&Layout[@s, @a], Layout[@ns, @na]) -> Result<(Layout[s + ns, na], usize), LayoutError>[s + ns + na - 1 <= isize::MAX]
                    requires a == 1)]
    fn extend(&self, next: Layout) -> Result<(Layout, usize), LayoutError>;

    #[flux_rs::spec(fn(&Layout[@s, @a]) -> Layout[s, a] requires a == 1)]
    fn pad_to_align(&self) -> Layout;
}

#[flux_rs::extern_spec(core::char)]
impl char {
    #[flux_rs::spec(fn(char, &mut [u8][@n]) -> &mut str{s: str_len(s) <= 4} requires 4 <= n)]
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    #[flux_rs::spec(fn(char) -> usize{v: 1 <= v && v <= 4})]
    fn len_utf8(self) -> usize;
}

#[flux_rs::extern_spec(core::num)]
impl usize {
    #[flux_rs::spec(fn(a: usize, b: usize) -> usize[if a * b <= usize::MAX { a * b } else { usize::MAX }])]
    fn saturating_mul(self, rhs: usize) -> usize;
}

#[flux_rs::extern_spec(alloc::alloc)]
#[flux_rs::spec(fn(Layout[@s, @a]) -> *mut{p: p.addr == 0 || readable(p, s)} u8 requires s > 0)]
unsafe fn alloc(layout: Layout) -> *mut u8;

#[flux_rs::extern_spec(alloc::alloc)]
#[flux_rs::spec(fn(*mut u8, Layout, new_size: usize) -> *mut{p: p.addr == 0 || readable(p, new_size)} u8
                requires 0 < new_size)]
unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8;

#[flux_rs::extern_spec(alloc::string)]
impl String {
    #[flux_rs::spec(fn(&String[@s]) -> usize[str_len(s)])]
    fn len(&self) -> usize;

    #[flux_rs::spec(fn(&String[@s]) -> usize{c: 0 <= str_len(s) && str_len(s) <= c && c <= 72057594037927934})]
    fn capacity(&self) -> usize;

    #[flux_rs::spec(fn(&String[@s]) -> &str[s])]
    fn as_str(&self) -> &str;
}
