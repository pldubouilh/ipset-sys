
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use anyhow::{bail, Result};
use std::ffi::CString;

struct ipset_sys {
    is : *mut ipset,
}

impl ipset_sys {
    pub fn init() -> Result<ipset_sys> {
        let is = unsafe {
            ipset_load_types();
            ipset_init()
        };

        // todo: printf

        if is.is_null() {
            bail!("can't init ipset");
        }

        Ok(ipset_sys{ is })
    }

    pub fn run(&mut self, cmd: &str) -> Result<()> {
        let cline = CString::new(cmd.to_string()).unwrap();
        let ret = unsafe { ipset_parse_line(self.is, cline.into_raw()) };

        if ret < 0 {
            bail!("failed executing ipset command")
        }

        Ok(())
    }
}

impl Drop for ipset_sys {
    fn drop(&mut self) {
        unsafe { ipset_fini(self.is) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() -> Result<()> {
        let mut i = ipset_sys::init()?;
        let _ = i.run("destroy bob");
        i.run("create bob hash:ip timeout 0")?;
        Ok(())
    }
}
