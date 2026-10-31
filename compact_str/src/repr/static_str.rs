use core::{mem, ptr, slice, str};

use super::{Repr, MAX_SIZE, STATIC_STR_MASK};

pub(super) const DISCRIMINANT_SIZE: usize = MAX_SIZE - mem::size_of::<&'static str>();

/// A buffer stored on the stack whose size is equal to the stack size of `String`
/// The last byte is set to 0.
#[derive(Copy, Clone)]
#[repr(C)]
#[flux_rs::refined_by(len: int)]
#[flux_rs::invariant(0 <= len && len <= 72057594037927934)]
pub(crate) struct StaticStr {
    #[flux_rs::field(ptr::NonNull<u8>{p: p.addr != 0 && p.base <= p.addr && len <= p.size})]
    ptr: ptr::NonNull<u8>,
    #[flux_rs::field(usize[len])]
    len: usize,
    #[allow(unused)]
    discriminant: [u8; DISCRIMINANT_SIZE],
}
static_assertions::assert_eq_size!(StaticStr, Repr);
static_assertions::assert_eq_align!(StaticStr, Repr);
static_assertions::assert_eq_size!(&'static str, (*const u8, usize));

impl StaticStr {
    #[flux_rs::spec(fn(text: &str[@t]) -> StaticStr[crate::flux_specs::str_len(t)])]
    #[inline]
    pub(crate) const fn new(text: &'static str) -> Self {
        let mut discriminant = [0; DISCRIMINANT_SIZE];
        discriminant[DISCRIMINANT_SIZE - 1] = STATIC_STR_MASK;

        Self {
            // SAFETY: `&'static str` must have a non-null, properly aligned
            // address
            ptr: unsafe { ptr::NonNull::new_unchecked(text.as_ptr() as *mut _) },
            len: text.len(),
            discriminant,
        }
    }

    #[flux_rs::spec(fn(&StaticStr[@n]) -> &str{t: crate::flux_specs::str_len(t) == n})]
    pub(super) const fn get_text(&self) -> &'static str {
        // SAFETY: `StaticStr` invariants requires it to be a valid str
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.ptr.as_ptr(), self.len)) }
    }

    /// # Safety
    /// * `len` bytes in the buffer must be valid UTF-8 and
    /// * `len` must be <= `self.get_text().len()`
    #[flux_rs::trusted(reason = "ASSUMED: `&str[..len]` has no length in flux; callers keep `len` in bounds")]
    #[flux_rs::spec(fn(self: &strg StaticStr[@n], len: usize) requires len <= n ensures self: StaticStr[len])]
    pub(super) unsafe fn set_len(&mut self, len: usize) {
        *self = Self::new(&self.get_text()[..len]);
    }
}
