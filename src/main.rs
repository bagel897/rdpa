use std::io;

use rdpa::adapters::{adapter::Adapter, btrfs::BtrfsAdapter};
fn main() {
    println!("Hello, world!");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    let t = BtrfsAdapter::new(&s).expect("Error creating adapter.");
    println!("{}", t.name());
}
