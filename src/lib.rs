//! A small crate to talk to ipset from rust.
//!
//! supports both regular `libipset` commands, and custom/faster handler for adding ipv4 to sets
//!
//! # Example
//!
//! ```rust
//! use ipset_sys::IpsetSys;
//!
//! fn main() {
//!     let mut is = IpsetSys::init().unwrap();
//!
//!     // regular libipset command
//!     is.run("create bob hash:ip timeout 3600").unwrap();
//!
//!     // custom ipv4 handler to add to a set
//!     let addr = std::net::Ipv4Addr::new(1, 4, 4, 4);
//!     is.add_v4("bob", addr).unwrap();
//!
//!     is.run("destroy bob").unwrap();
//! }
//! ```
//!
use std::ffi::{c_void, CString};
use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IpsetSysError {
    #[error("can not init ipset")]
    CantInit,
    #[error("can not execute ipset command")]
    CantExecuteCommand,
    #[error("command invalid")]
    CommandInvalid,
    #[error("argument too long")]
    ArgTooLong,
    #[error("invalid timeout, should be 0 < t < 2147483")]
    InvalidTimeout,
    #[error("invalid command - can not parse to CString")]
    InvalidCommand {
        #[from]
        source: std::ffi::NulError,
    },
}

mod bindings;

macro_rules! ensure_ptr {
    ( $x:expr ) => {
        if $x.is_null() {
            return Err(IpsetSysError::CantExecuteCommand);
        }
    };
}

pub struct IpsetSys {
    is: *mut bindings::ipset,
}

impl IpsetSys {
    /// init ipset handler. will fail upon unsufficient rights, or lacking library.
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

    /// normal libipset command handler. parses string commands as directed by CLI
    /// see `man ipset` for more details
    ///
    /// # Example
    ///
    /// ```rust
    ///   let mut is = ipset_sys::IpsetSys::init().unwrap();
    ///   is.run("create bob hash:ip timeout 3600").unwrap();
    ///   is.run("add bob 1.4.4.4").unwrap();
    /// ```
    pub fn run(&mut self, cmd: &str) -> Result<(), IpsetSysError> {
        let ccmd = CString::new(cmd.to_string())?;
        let ret = unsafe { bindings::ipset_parse_line(self.is, ccmd.into_raw()) };

        if ret < 0 {
            return Err(IpsetSysError::CantExecuteCommand);
        }

        Ok(())
    }

    /// custom ipset command to add an ipv4 to a given set.
    /// this command directly issues ipset commands without going through the CLI parser, etc...
    ///
    /// # Example
    ///
    /// ```rust
    ///   let mut is = ipset_sys::IpsetSys::init().unwrap();
    ///   let addr = Ipv4Addr::new(1, 4, 4, 4);
    ///   is.add_v4("bob", addr).unwrap();
    /// ```
    pub fn add_v4(&mut self, set: &str, target: Ipv4Addr) -> Result<(), IpsetSysError> {
        let set_ptr = parse_user_string(set)?;
        let target = parse_ipv4(target);
        let target_ptr = &target as *const _ as *const c_void;

        let nf_ipv4_ptr = &bindings::NFPROTO_IPV4 as *const _ as *const c_void;

        unsafe {
            let (sess, data) = self.get_sess_and_data()?;
            set_data(data, bindings::ipset_opt_IPSET_SETNAME, set_ptr)?;
            set_data(data, bindings::ipset_opt_IPSET_OPT_TYPE, nf_ipv4_ptr)?;
            set_data(data, bindings::ipset_opt_IPSET_OPT_FAMILY, nf_ipv4_ptr)?;
            set_data(data, bindings::ipset_opt_IPSET_OPT_IP, target_ptr)?;

            run_cmd(sess, bindings::ipset_cmd_IPSET_CMD_ADD)?;
        }
        Ok(())
    }

    unsafe fn get_sess_and_data(
        &mut self,
    ) -> Result<(*mut bindings::ipset_session, *mut bindings::ipset_data), IpsetSysError> {
        let sess = bindings::ipset_session(self.is);
        ensure_ptr!(sess);
        let data = bindings::ipset_session_data(sess);
        ensure_ptr!(data);
        Ok((sess, data))
    }
}

unsafe fn set_data(
    data: *mut bindings::ipset_data,
    cmd: bindings::ipset_opt,
    val: *const c_void,
) -> Result<(), IpsetSysError> {
    let ret = bindings::ipset_data_set(data, cmd, val);
    if ret < 0 {
        return Err(IpsetSysError::CantExecuteCommand);
    }
    Ok(())
}

unsafe fn run_cmd(
    sess: *mut bindings::ipset_session,
    cmd: bindings::ipset_cmd,
) -> Result<(), IpsetSysError> {
    let ret = bindings::ipset_cmd(sess, cmd, 0);
    if ret < 0 {
        return Err(IpsetSysError::CantExecuteCommand);
    }
    Ok(())
}

fn parse_user_string(set: &str) -> Result<*const c_void, IpsetSysError> {
    if set.len() as u32 > bindings::IPSET_MAXNAMELEN {
        return Err(IpsetSysError::ArgTooLong);
    }
    let cset = CString::new(set.to_string())?;
    let ptr = cset.into_raw() as *const c_void;
    Ok(ptr)
}

fn parse_ipv4(target: Ipv4Addr) -> libc::in_addr {
    let addr_u32: u32 = target.into();
    libc::in_addr {
        s_addr: addr_u32.to_be(),
    }
}

impl Drop for IpsetSys {
    fn drop(&mut self) {
        unsafe { bindings::ipset_fini(self.is) };
    }
}
