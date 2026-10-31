#[cfg(flux)] extern crate flux_core;
pub struct D<'a> { c: core::str::Chars<'a> }
impl Iterator for D<'_> {
    type Item = char;
    fn next(&mut self) -> Option<char> { self.c.next() }
}
