use std::ffi::CString;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IpsetSysError {
    #[error("can not init ipset")]
    CantInit,
    #[error("can not execute ipset command")]
    CantExecuteCommand,
    #[error("invalid command - can not parse to CString")]
    InvalidCommand {
        #[from]
        source: std::ffi::NulError,
    },
}

mod bindings;

pub struct IpsetSys {
    is: *mut bindings::ipset,
}

impl IpsetSys {
    pub fn init() -> Result<IpsetSys, IpsetSysError> {
        let is = unsafe {
            bindings::ipset_load_types();
            bindings::ipset_init()
        };

        // todo: printf

        if is.is_null() {
            return Err(IpsetSysError::CantInit);
        }

        Ok(IpsetSys { is })
    }

    pub fn run(&mut self, cmd: &str) -> Result<(), IpsetSysError> {
        let ccmd = CString::new(cmd.to_string())?;
        let ret = unsafe { bindings::ipset_parse_line(self.is, ccmd.into_raw()) };

        if ret < 0 {
            return Err(IpsetSysError::CantExecuteCommand);
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
    fn init() {
        let mut i = IpsetSys::init().unwrap();
        let _ = i.run("destroy bob");
        i.run("create bob hash:ip timeout 0").unwrap();
    }
}
