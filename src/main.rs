mod keylogger;
use keylogger::KeyLogger;

fn main() {
    loop {
        println!("{:?}", KeyLogger::get_keys());
    }
}