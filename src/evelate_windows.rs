use std::io;
use std::ptr::null_mut;
use std::ffi::CString;
use std::os::windows::ffi::OsStringExt;

use crate::utils::*;
use crate::winapi::*;

// elevate privileges
pub fn evelate_privileges() -> Result<(), io::Error> {
    let mut htk: HANDLE = null_mut();
    let mut tkp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Attributes: SE_PRIVILEGE_ENABLED,
            ..Default::default()
        }],
    };
    if FALSE == unsafe{OpenProcessToken(GetCurrentProcess(), 
        TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut htk)} {
        println!("open process token failed");
        return Err(get_last_error());
    }

    if FALSE == unsafe{LookupPrivilegeValueA(null_mut(), 
        CString::new(SE_DEBUG_NAME)?.as_ptr() as _, &mut tkp.Privileges[0].Luid)} {
        println!("lookup privilege value failed");
        return Err(get_last_error());
    }

    if FALSE == unsafe{AdjustTokenPrivileges(htk, FALSE, &mut tkp, 0, null_mut(), null_mut())} {
        println!("adjust token privilege failed");
        return Err(get_last_error());
    }

    Ok(())
}