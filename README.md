# win-keylogger
Window's Win32 API provides programs with the fastest possible access to OS and hardware data.
In addition to functions like networking functions, driver access, and COM access, the Win32 API 
lets programs query user input (i.e. they keyboad and mouse). This keylogger is built on top of a 
made-from-scratch keyboard listener that uses the Win32 API to query the state of the keyboard 
(i.e. the state of all 256 virtual-key codes) as a 256-byte array and then uses that data to return
an array of all the keys that are current being pressed. This vector may be empty.

The keyboard listening process is as follows:
1. Obtain the keyboard layout
2. Get the state of the keyboard \[u8; 256\]
3. Query the state of each virtual-key using GetAsyncKeyState()
    3A. If active, translate the virtual key into the corresponding Unicode char
    5B. Append the unicode character to the list of active keys
4. Return the list of active keys

# Todo
1. Add case sensitivity
# Known Bugs
1. Currently, 'pressed_keys()' only returns the first byte of the buffer from
   [_ToUnicodeEx()_](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-tounicodeex), which causes mistranslations because of [Unicode Surrogate Pairs](https://docs.microsoft.com/en-us/windows/win32/intl/surrogates-and-supplementary-characters)
   (characters that take up more than one char (one byte) of space). For more information, read 
   [this StackOverflow question](https://stackoverflow.com/questions/51001150/what-are-surrogate-characters-in-utf-8).
   > IMPORTANT: the `char` datatype is 4-bytes in length in Rust, unlike in C/C++ where it is 1 byte
2. I don't think 'VkKeyScanA()' is properly interpreting the characters into virtual-key codes,
   given initial testing results.

# Windows API Reference Links
- [Overview: Keyboard and Mouse Input Functions](https://docs.microsoft.com/en-us/windows/win32/api/_inputdev/)
- [GetKeyboardState()](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardstate) 
- [GetLastError()](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
- [FormatEssage()](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage)
- [Table of Virtual Keycodes](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
- [MapVirtualKeyW()](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapvirtualkeyw)
- [ToUnicode()](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-tounicode)
# Implementation References
- [YXL76's keyboard_query crate](https://github.com/YXL76/keyboard_query/blob/master/src/lib.rs)
- [Deadblackclover's keylogger](https://github.com/deadblackclover/keylogger/blob/master/src/keylogger.rs)
