//For KeyboardListener
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardState, GetAsyncKeyState, VkKeyScanA};

//For KeyboardReadError
use std::fmt::{Debug, Display};
use windows::Win32::Foundation::{GetLastError};

/*
    Keyboard listener. Used for retrieving keystrokes 
    from the kernal via Win32 API calls.

    Important notes:
    ================
    - Difference between GetKeyState() and GetKeyStateAsync(): https://stackoverflow.com/questions/17770753/getkeystate-vs-getasynckeystate-vs-getch
        > For some reason, just GetKeyState() doesn't seem to be fast enough to display on screen (from initial debugging)

    - Win32 documentation suggests checking if the key is pressed with 'GetKeyState() & 0x8000',
      where the 0 is implicitly evaluated to a boolean, but in Rust you can't legally make that comparison
      without an int cast and then adding '!= 0'. So, I simplified it to '== -32737', which does the same thing.
        > Reference: https://docs.microsoft.com/en-us/windows/win32/learnwin32/keyboard-input#keyboard-state

*/
pub struct KeyboardListener;

impl KeyboardListener
{
    fn get_keyboard_state() -> Result<[u8; 256], KeyboardReadError> {
        unsafe {
            let mut keyboard_state = [0u8; 256];
            if bool::from(GetKeyboardState(&mut keyboard_state)) {
                return Ok(keyboard_state);
            }
            
            Err(KeyboardReadError)
        }
    }

    /*
        Translate from C++:
        HKL keyboard_layout = LoadKeyboardLayout("00000409", KLF_ACTIVATE);
        SHORT virtual_key_code = VkKeyScanEx(key, keyboard_layout);
        return GetAsyncKeyState(virtual_key_code) & 0x8000;
    */
    pub fn is_pressed(key: char) -> bool {
        unsafe{
            let virtual_key_code = key as i32; //VkKeyScanA(key as CHAR);
            GetAsyncKeyState(virtual_key_code) == -32767
        }
    }

    /*
    
    */
    pub fn pressed_keys() -> Vec<char> {
        let mut pressed_keys = Vec::new();

        unsafe {
            for virtual_key_code in 0..256 {
                    if GetAsyncKeyState(virtual_key_code) == -32767 { 
                        pressed_keys.push(virtual_key_code as u8 as char); 
                    }
                }
            }

        pressed_keys
    }
}


/* 
    Implementation of 'KeyboardReadError', used when the Win32 API
    cannot return the state of the key(board)
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
