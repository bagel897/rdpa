#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rdpa::adapters::{adapter::Adapter, btrfs::BtrfsAdapter};

    #[test]
    fn basic_test() {
        let t = BtrfsAdapter::new(&String::from_str("/").expect("Error")).expect("Error");
        assert_eq!(t.name(), "/");
    }
}
