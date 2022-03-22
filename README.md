# win-keylogger
The Windows API provides access to reading the state of the keyboard and mouse
via [_user32.dll_](https://docs.microsoft.com/en-us/windows/win32/api/_inputdev/).
The KeyLogger class is interested in maining the state of the entire state of
the keyboard (`GetKeyboardState()`) instead of just `GetKeyStateAsync()`.

Since `GetKeyboardState()` is an IO-bound operation (i.e. it is an I/O operation
that takes considerable time and prevents other work), the `KeyLogger.get_keyboard_state()` should be either asynchonous or multi-threaded.

# Plans
1. Add support for querying individual keys using byte wizardry
2. Add a buffer to the keylogger so it can record whole words, not just
individual characters. That way, regexs can be applied to parse for emails,
passwords, etc.
# To-do
1. Rework `get_keyboard_state()` to be asynchonous (maybe make keyboard_state a part of the 'KeyLogger' struct?)
2. Make 'keyboard_state' from 'KeyLogger.get_keyboard_state' a byte array 
instead of [u8; 256];
3. Add error strings to 'KeyboardReadError' via Windows API function 'FormatMessage'

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
