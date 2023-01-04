use std::io;
pub trait Adapter {
    fn new(path: &String) -> Result<Self, io::Error>
    where
        Self: Sized;
    fn name(&self) -> &str;
}
