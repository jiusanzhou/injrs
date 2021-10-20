use std::fmt;

const UNKNOWN_ERROR_MESSAGE: &'static str = "unknown error";

#[derive(Debug, Clone)]
pub struct Win32Error {
    // Code
    pub code: u32,
    pub message: Option<String>,
}

impl Win32Error {

    pub fn new() -> Self {
        Self{code: 0, message: None}
    }

}

unsafe impl Sync for Win32Error {}
    
unsafe  impl Send for Win32Error {}

impl fmt::Display for Win32Error {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.message.as_ref() {
            Some(s) => format!("{}: {}", self.code, s),
            None => format!("{}: {}", self.code, UNKNOWN_ERROR_MESSAGE),
        }.fmt(f)
    }

}