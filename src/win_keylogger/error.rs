extern crate windows;

use windows::api::{GetLastError};
use std::fmt::{Debug, Display};

mod win_keylogger {
    struct KeyboardReadError; 

    /*
        GetKeyboardState() returns 0 on failure, but
        the error details are contained within the error code
        of the caller.
    */
    impl Debug for key_read_error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            unsafe {
                let error_code = GetLastError();
                
                //TODO - use ::Win32::Diagnostics::Debug::FormatMessage to get error string
                let error_desc = format!("{}", error_code);
                write!(f, "Error reading keyboard. Error code {} ({})", error_code, error_desc)
            }
        } 
    }

    impl Display for key_read_error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Error: keyboard state could not be read")
        } 
    }
}