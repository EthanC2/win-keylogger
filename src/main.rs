mod keyboard_listener;
use keyboard_listener::{KeyboardListener};

fn main() {
    println!("Starting!");
    loop {
        let kl = KeyboardListener::new();
        let keys = kl.pressed_keys();
    
        if !keys.is_empty() {
            println!("{:?}", keys);
        }

        /*
        if KeyboardListener::is_pressed('a') {
            println!("a is pressed!");
        }

        if KeyboardListener::is_pressed('b') {
            println!("b is pressed!");
        }

        if KeyboardListener::is_pressed('z') {
            println!("z is pressed!");
        }

        if KeyboardListener::is_pressed('A') {
            println!("A is pressed!");
        }
        */
    }
}
