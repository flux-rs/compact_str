use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct E;
impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("e") }
}
impl Error for E {}
pub fn boxed() -> Box<dyn Error + Send + Sync> {
    Box::new(E)
}
