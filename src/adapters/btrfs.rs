use crate::adapters::adapter::Adapter;
use nix::sys::statfs::{statfs, BTRFS_SUPER_MAGIC};
use nix::sys::statvfs::statvfs;

use std::io::{self, ErrorKind};
pub struct BtrfsAdapter {
    path: Box<String>,
}
impl Adapter for BtrfsAdapter {
    fn new(path: &String) -> Result<Self, io::Error> {
        println!("{}", path);
        let this = statfs(path.as_str());
        let t = match this {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        };

        if BTRFS_SUPER_MAGIC != t.filesystem_type() {
            return Err(io::Error::new(ErrorKind::Other, "Not a BTRFS filesystem."));
        }
        return Ok(BtrfsAdapter {
            path: Box::new(path.clone()),
        });
    }

    fn name(&self) -> &str {
        return &self.path;
    }
}
