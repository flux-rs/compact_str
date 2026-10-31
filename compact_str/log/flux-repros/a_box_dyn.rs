use std::error::Error;
pub fn drop_markers(e: Box<dyn Error + Send + Sync>) -> Box<dyn Error> {
    e
}
