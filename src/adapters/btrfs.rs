use crate::adapters::adapter::Adapter;
use nix::sys::statfs::BTRFS_SUPER_MAGIC;
use nix::sys::statvfs::statvfs;
use std::io::{self, ErrorKind};
use std::path::Path;
pub struct BtrfsAdapter {
    path: &Path,
}
impl Adapter for BtrfsAdapter {
    fn new(path: &Path) -> Result<Self, io::Error> {
        let this = statvfs(path);
        let t = match this {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        };

        if BTRFS_SUPER_MAGIC.0 != t.filesystem_id().try_into().unwrap() {
            return Err(io::Error::new(ErrorKind::Other, "Not a BTRFS filesystem."));
        }
        return Ok(BtrfsAdapter { path });
    }

    fn name(&self) -> &str {
        return self
            .path
            .to_str()
            .expect("Could not convert path to string.");
    }
}
