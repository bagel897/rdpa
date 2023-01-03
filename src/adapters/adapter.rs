use std::{io, path::Path};
pub trait Adapter {
    fn new(path: &Path) -> Result<Self, io::Error>
    where
        Self: Sized;
    fn name(&self) -> &str;
}
