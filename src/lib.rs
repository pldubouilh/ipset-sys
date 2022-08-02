use anyhow::{bail, Result};
use std::ffi::CString;

mod bindings;

pub struct IpsetSys {
    is: *mut bindings::ipset,
}

impl IpsetSys {
    pub fn init() -> Result<IpsetSys> {
        let is = unsafe {
            bindings::ipset_load_types();
            bindings::ipset_init()
        };

        // todo: printf

        if is.is_null() {
            bail!("can't init ipset");
        }

        Ok(IpsetSys { is })
    }

    pub fn run(&mut self, cmd: &str) -> Result<()> {
        let cline = CString::new(cmd.to_string()).unwrap();
        let ret = unsafe { bindings::ipset_parse_line(self.is, cline.into_raw()) };

        if ret < 0 {
            bail!("failed executing ipset command")
        }

        Ok(())
    }
}

impl Drop for IpsetSys {
    fn drop(&mut self) {
        unsafe { bindings::ipset_fini(self.is) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() -> Result<()> {
        let mut i = IpsetSys::init()?;
        let _ = i.run("destroy bob");
        i.run("create bob hash:ip timeout 0")?;
        Ok(())
    }
}
