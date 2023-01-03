use std::{path::Path, error::Error}; 
pub trait Adapter {
    fn new(path: &'static Path) -> Result<Self, Error>;
    fn name(&self) -> &'static str; 
}
