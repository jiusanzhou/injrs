#[allow(unused_imports)]

use crate::winapi::*;
use crate::utils::*;

use std::io;
use std::mem::{self, MaybeUninit};

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;


pub type ProcessHandle = HANDLE;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub handle: ProcessHandle,
}

// Create and init the process struct
impl Process {

    pub fn new(handle: ProcessHandle, pid: u32, name: &str) -> Self {
        Self {
            pid: pid,
            name: name.into(),
            handle,
        }
    }

    pub fn from_pid(pid: u32) -> Option<Self> {

        // open process by pid, bacause we need to write message
        // so for simple just open as all access flag
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };
        if handle.is_null() {
            return None;
        }

        let name = get_process_name(handle);

        Some(Self::new(handle, pid, name.as_str()))
    }

    pub fn from_pid_and_name(pid: u32, name: &str) -> Option<Self> {
        // open process by pid, bacause we need to write message
        // so for simple just open as all access flag
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };
        if handle.is_null() {
            return None;
        }
        
        Some(Self::new(handle, pid, name))
    }

    pub fn find_first_by_name(name: &str) -> Option<Self> {
        match find_process_by_name(name).unwrap_or_default().first() {
            // TODO: ugly, shoudl implement copy trait for process
            Some(v) => Process::from_pid(v.pid),
            None => None
        }
    }

    pub fn find_all_by_name(name: &str) -> Vec<Self> {
        match find_process_by_name(name) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        }
    }

    pub fn from_handle(handle: ProcessHandle) -> Self {

        let pid = unsafe { GetProcessId(handle) as u32 };

        let name = get_process_name(handle);

        Self {pid, name, handle}
    }

}

impl Process {
    pub fn close(&self) -> io::Result<()> {
        if self.handle.is_null() {
            return Ok(());
        }
        let result = unsafe{ CloseHandle(self.handle) };
        if result != 0 {
            return Ok(());
        }
        Err(get_last_error())
    }

    pub fn find_module_by_name(dllname: &str) -> Option<MODULEENTRY32> {
        todo!()
    }

    pub fn is_wow64(&self) -> Result<bool, io::Error> {
        let mut is_wow64 = MaybeUninit::uninit();
        let r = unsafe{IsWow64Process(self.handle, is_wow64.as_mut_ptr())};
        if r == FALSE {
            return Err(get_last_error());
        }
        Ok(unsafe{is_wow64.assume_init()} == TRUE)
    }

}

impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub fn get_process_name(handle: ProcessHandle) -> String {
    let mut buf = [0u16; MAX_PATH + 1];
    unsafe {
        GetModuleBaseNameW(handle, 0 as _, buf.as_mut_ptr(),  MAX_PATH as DWORD + 1);
        return wchar_to_string(&buf)
    };
}

pub fn get_process_path(handle: ProcessHandle) -> String {
    let mut buf = [0u16; MAX_PATH + 1];
    unsafe {
        GetModuleFileNameExW(handle, 0 as _, buf.as_mut_ptr(), MAX_PATH as DWORD + 1);
        return wchar_to_string(&buf);
    }
}

// TODO: accept callback function
pub fn find_process_by_name(name: &str) -> Result<Vec<Process>, io::Error> {
    let handle = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0 as _)
    };

    if handle.is_null() || handle == INVALID_HANDLE_VALUE {
        return Err(get_last_error());
    }

    // the result to store process list
    let mut result: Vec<Process> = Vec::new();

    let mut _name: String;

    // can't reuse
    let mut entry: PROCESSENTRY32 = unsafe { ::std::mem::zeroed() };
    entry.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

    while 0 != unsafe { Process32Next(handle, &mut entry) } {
        // extract name from entry
        _name = char_to_string(&entry.szExeFile);
        // clean entry exefile filed
        entry.szExeFile = unsafe { ::std::mem::zeroed() };

        if name.len() > 0 && !_name.contains(name) {
            // ignore if name has set but not match the exefile name
            continue;
        }
        // parse process and push to result vec
        // TODO: improve reuse the name and other information
        match Process::from_pid_and_name(entry.th32ProcessID, _name.as_str()) {
            Some(v) => result.push(v),
            None => {},
        }

    }

    Ok(result)
}

fn char_to_string(chars  : &[i8]) -> String {
    chars.into_iter().map(|c| { *c as u8 as char }).collect()
}

pub fn wchar_to_string(slice: &[u16]) -> String {
    match slice.iter().position(|&x| x == 0) {
        Some(pos) => OsString::from_wide(&slice[..pos])
            .to_string_lossy()
            .into_owned(),
        None => OsString::from_wide(slice).to_string_lossy().into_owned(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize() {
        Process::new(0 as _, 0, "");
    }

    #[test]
    fn get_first_process() {
        match Process::find_first_by_name("Code.exe") {
            Some(v) => {
                println!("get the first process: {}", v.name);
            },
            None => {},
        }
    }

    #[test]
    fn get_process() {
        println!("get process:");
        match find_process_by_name("Code.exe") {
            Ok(v) => {
                println!("get process count: {}", v.len());
                for x in &v {
                    println!("{} {}", x.pid, x.name);
                }
            },
            Err(e) => eprintln!("find process by name error: {}", e)
        }
    }
}