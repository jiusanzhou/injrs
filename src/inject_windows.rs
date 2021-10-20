use std::io;
use std::mem::{self, MaybeUninit};
use std::ptr::null_mut;
use std::path::Path;

use crate::utils::*;
use crate::winapi::*;
use crate::process_windows::*;

// https://gist.github.com/sum-catnip/00491d030f69918e96369ce900b24d52

// open the process
// call remote thread

pub trait InjectorExt {
    fn is_injected(&self, dll: &str) -> Result<bool, io::Error>;
    fn inject(&self, dll: &str) -> Result<(), io::Error>;
    fn eject(&self, dll: &str) -> Result<(), io::Error>;
}

impl InjectorExt for Process {

    fn is_injected(&self, dll: &str) -> Result<bool, io::Error> {
        todo!()
    }

    fn inject(&self, dll: &str) -> Result<(), io::Error> {
        // make sure dll exits
        let fullpath = Path::new(dll).canonicalize();
        if fullpath.is_err() {
            return Err(fullpath.unwrap_err());
        }
        let fullpath = fullpath.unwrap();
        let dll = fullpath.to_str().unwrap();

        let path_wstr = to_wide_string(dll);

        // length from wide string and add the last \0
        let path_len = path_wstr.len() * 2 + 1;

        // alloc memorry in the process to store dll path name
        let r_path_addr = unsafe{VirtualAllocEx(self.handle, null_mut(), path_len,
            MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE)};

        // check if the addr is null
        if r_path_addr.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "alloc memorry failed"));
        }

        // write dll path to the remote
        let r = unsafe{WriteProcessMemory(self.handle, r_path_addr,
            path_wstr.as_ptr() as _, path_len, null_mut())};

        // check written result
        if r == FALSE {
            return Err(io::Error::new(io::ErrorKind::Other, "write data to memorry failed"));
        }

        // get method address from process
        let r_func_addr = unsafe{GetProcAddress(
            GetModuleHandleA("kernel32.dll\0".as_ptr() as _),
            "LoadLibraryW\0".as_ptr() as _,
        )};

        // check if the addr is null
        if r_func_addr.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "get load func memorry failed"));
        }

        // create remote thread to call load library
        let t_handle = unsafe{CreateRemoteThread(
            self.handle,
            null_mut(),
            0,
            Some(mem::transmute(r_func_addr)),
            r_path_addr,
            0,
            null_mut()
        )};
        if t_handle.is_null() {
            println!("create remote thread failed");
            return Err(get_last_error());
        }

        // wait for thread
        let r = unsafe{WaitForSingleObject(t_handle, 100)}; // INFINITE
        if r == WAIT_FAILED {
            // println!("==== wait for single object");
            return Err(get_last_error());
        }

        // // try to get exit code of thread
        // let mut v = MaybeUninit::uninit();
        // let r = unsafe {GetExitCodeThread(t_handle, v.as_mut_ptr())};
        // if r == FALSE {
        //     return Err(get_last_error());
        // }
        // let v = unsafe {v.assume_init()};

        // release the r_path_addr
        unsafe{VirtualFreeEx(self.handle, r_path_addr, 1, MEM_DECOMMIT)};

        // close the thread handle
        unsafe{CloseHandle(t_handle)};

        Ok(())
    }

    fn eject(&self, dll: &str) -> Result<(), io::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::process_windows::*;
    use super::InjectorExt;
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_path() {
        println!("{:?}", ::std::env::current_dir().unwrap());
        // target/i686-pc-windows-msvc/release/example.dll
        let path = Path::new("./example").canonicalize().unwrap();
        for entry in fs::read_dir(path).unwrap() {
            println!("===> {:?}", entry.unwrap());
        }
    }

    #[test]
    fn inject_example() {
        use crate::evelate_windows::*;

        let _ = evelate_privileges();

        println!("Hello injector!");
        let p = Process::find_first_by_name("WeChat.exe").unwrap();
        let r = p.inject("./example/target/i686-pc-windows-msvc/release/example.dll");
        match r {
            Err(e) => println!("inject error: {}", e),
            Ok(_) => println!("inject success"),
        }
    }
}