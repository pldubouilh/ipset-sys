# ipset-sys

[crate.io](https://crates.io/crates/ipset-sys)

bindings to libipset.

a more comprehensive library directly talking to netlink will hopefully be available soon after !

```rs
use ipset_sys::IpsetSys;

fn main() {
    let mut is = IpsetSys::init().unwrap();
    is.run("create mylist hash:ip timeout 0").unwrap();
}
```