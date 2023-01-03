use std::{io, path::Path};

use rdpa::adapters::{adapter::Adapter, btrfs::BtrfsAdapter};
fn main() {
    println!("Hello, world!");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    let p = Path::new(&s);
    let t = BtrfsAdapter::new(p).expect("Error creating adapter.");
    println!("{}", t.name());
}
