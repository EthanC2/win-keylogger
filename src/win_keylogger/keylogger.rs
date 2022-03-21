extern crate windows;
use windows::user32::GetKeyboardState;

mod win_keylogger {
    
    pub struct KeyLogger;

    impl KeyLogger {

        /*
            "Copies the status of the 256 virtual keys to the specified buffer.
            What are virtual keys?: https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

            'windows::user32::GetKeyboardState()' Notes
            ===========================================
            @parameter 'lpKeyState': a 256-byte array that receives the status of the 256 virtual keys
            @return_value: if the function succeeds, the return value is non-zero.
        */

        //TODO: update 'keyboard_state' to use a byte array from std::bytes
        pub fn get_keyboard_state() -> Result<[u8; 256], KeyboardReadError> {
            let mut keyboard_state = [0u8; 256];
            
            unsafe {
                match GetKeyboardState(&mut keyboard_state) {
                    0 => Err(KeyboardReadError), 
                    _ => Ok(keyboard_state)
                }
            }
        }
    }
}