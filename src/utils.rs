use std::io;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt as _;

#[inline(always)]
pub fn get_last_error() -> io::Error {
    io::Error::last_os_error()
}

pub fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}