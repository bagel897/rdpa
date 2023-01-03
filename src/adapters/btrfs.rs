use std::path::Path;

use crate::adapters::adapter::Adapter;

struct BtrfsAdapter {} 
impl Adapter for BtrfsAdapter  {
    fn new(path: &'static Path) -> Result<Self> {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
