//For KeyboardListener
use windows::Win32::Foundation::{CHAR};       //basic Win32 types (e.g. HANDLE, HWND, DWORD, etc.)
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardState, GetAsyncKeyState, GetKeyboardLayout, VkKeyScanExA, ToUnicodeEx, MapVirtualKeyExA}; //input functions
use windows::Win32::UI::TextServices::{HKL};  //represents keyboard layout (for interpreting virtual-key codes)
use windows::Win32::UI::WindowsAndMessaging::{MAPVK_VK_TO_VSC};

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
pub struct KeyboardListener
{
    keyboard_layout: HKL,
}

impl KeyboardListener
{
    /*
        [Function]: 'new()'
        [Summary] Initializes the 'KeyboardListener' struct with the appropriate system locale
                  for properly interpreting virtual-key codes.
        [returns: KeyboardListener] a new instance of the 'KeyboardListener' struct
    */
    pub fn new() -> KeyboardListener
    {
        unsafe {
            KeyboardListener{ keyboard_layout: GetKeyboardLayout(0) }
        }
    }

    /*
        
        [function]: 'is_pressed()'
        [summary]: Returns true if the given key is pressed.
        [param 'key': char]: key - the key to check
        [returns: bool]: true if the key is pressed, false otherwise

        Important notes:
        ================
        - In order to get the proper virtual-key code from a char, you have to interpret 
          it with VkKeyScanExA using the current keyboard layout
            > https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-vkkeyscanexa

        - The only parameter for 'GetKeyboardLayout()' the ID of the thread to query for the 
          keyboard layout. The thread '0' indicates the current thread
            > https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayout
    */
    pub fn is_pressed(&self, key: char) -> bool {
        unsafe{
            let virtual_key_code = VkKeyScanExA(CHAR(key as u8), self.keyboard_layout) as i32;
            GetAsyncKeyState(virtual_key_code) == -32737
        }
    }

    /*
        [function]: 'pressed_keys()'
        [summary]: Loops through all 256 virtual-key codes and checks if they are pressed.
        [returns: Vec<char>]: a vector of all the keys that are currently pressed 

        The process of interpreting a virtual-key code as a Unicode character is as follows:
        1. Get the state of the keyboard (a 256-byte array) representing the states of the virtual-keys
        2. For each virtual-key code, check if it is pressed
        3. If pressed, get the unicode equivalent of the character using 
           'ToUnicodeEx()' using the current keyboard layout and the virtual-key code
        4. Add the character to the list of pressed keys 

        TODO: Add support for Unicode surrogate pairs and case sensitivity

        Important notes:
        ================
        -  I'm fucking dumb. I have no idea how long the buffer for 'unicode_char' needs to be, since the 
           original documentation for C++ has the datatpe as 'WCHAR' (which is a 16-bit unsigned integer),
           but in Rust there is no WCHAR type and for some reason the 'windows' create doesn't 
           support it (it does not show in the documentation under Win32::Foundation or elsewhere).
    */
    pub fn pressed_keys(&self) -> Vec<char> {
        let mut pressed_keys = Vec::new();

        unsafe {
            //Get all 256 virtual-key codes
            let mut keyboard_state = [0u8; 256];
            GetKeyboardState(&mut keyboard_state);

            for virtual_key_code in 0..256u32 {
                    if GetAsyncKeyState(virtual_key_code as i32) == -32767 { 
                        let mut unicode_char: [u16; 20] = [0; 20];
                        let scan_code = MapVirtualKeyExA(virtual_key_code, MAPVK_VK_TO_VSC, self.keyboard_layout);
                        ToUnicodeEx(virtual_key_code as u32, scan_code, &keyboard_state, &mut unicode_char, 1, self.keyboard_layout);
                        pressed_keys.push(unicode_char[0] as u8 as char);  //as u8 as char
                    }
                }
            }

        pressed_keys
    }
}