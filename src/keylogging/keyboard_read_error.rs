use std::fmt::{Debug, Display};

//Used when the Win32 API returns an error on 'GetKeyboardState()'
pub struct KeyboardReadError;


impl Debug for KeyboardReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            //TODO: actually update these lmao
            let error_code = 69;
            let error_desc = "huh";
            write!(f, "Win32 API Error on reading keyboard state: error code {} ({})", error_code, error_desc)
        }
    }
}

impl Display for KeyboardReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: keyboard state could not be read from system [Win32 API Error]")
    }
}