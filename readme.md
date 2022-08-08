# ipset-sys

[crate.io](https://crates.io/crates/ipset-sys)

A small crate to talk to ipset from rust.

supports both regular `libipset` commands, and custom/faster handler for adding ipv4 to sets

### Example

```rust
    use ipset_sys::IpsetSys;

    fn main() {
        let mut is = IpsetSys::init().unwrap()

        // regular libipset command
        is.run("create bob hash:ip timeout 3600").unwrap()

        // custom ipv4 handler to add to a set
        let addr = std::net::Ipv4Addr::new(1, 4, 4, 4);
        is.add_v4("bob", addr).unwrap()

        is.run("destroy bob").unwrap();
 }
 ```