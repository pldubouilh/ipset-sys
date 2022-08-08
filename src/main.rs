use std::net::Ipv4Addr;

fn main() {
    let mut is = ipset_sys::IpsetSys::init().unwrap();

    // regular libipset command
    is.run("create bob hash:ip timeout 3600").unwrap();

    // custom ipv4 handler to add to a set
    let addr = Ipv4Addr::new(1, 4, 4, 4);
    is.add_v4("bob", addr).unwrap();

    is.run("destroy bob").unwrap();
}
