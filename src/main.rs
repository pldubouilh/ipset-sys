fn main() {
    let mut is = ipset_sys::IpsetSys::init().unwrap();
    is.run("create bob hash:ip timeout 0").unwrap();
}
