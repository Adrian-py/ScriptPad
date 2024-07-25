use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for key_press in io::stdin().bytes() {
            match key_press {
                Ok(key_press) => {
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
                        break;
                    }
                }
                Err(err) => println!("Error: {:?}", err),
            }
        }
        disable_raw_mode().unwrap();
    }
}
