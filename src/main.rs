mod keyboard_listener;
use keyboard_listener::{KeyboardListener};

fn main() {
    println!("Starting!");
    loop {
        //println!("{:?}", KeyboardListener::pressed_keys());
        if KeyboardListener::is_pressed('g') {
            println!("G is pressed!");
        }
    }
}
