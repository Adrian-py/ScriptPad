use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for key_press in io::stdin().bytes() {
        let key_press = key_press.unwrap();
        let char: char = key_press as char;
        if char.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", key_press);
        } else {
            println!(
                "Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r",
                key_press, char
            );
        }
        if char == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
