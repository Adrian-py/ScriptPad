use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent};
use crossterm::queue;
use crossterm::style::{Color, PrintStyledContent, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_exit: false }
    }

    pub fn draw_rows(&self) -> std::io::Result<()> {
        match crossterm::terminal::size() {
            Ok((col, _)) => {
                let tilde = "~ ".with(Color::Green);
                for i in 0..col {
                    queue!(std::io::stdout(), MoveTo(0, i), PrintStyledContent(tilde))?;
                }
                queue!(std::io::stdout(), MoveTo(2, 0))?;
                std::io::stdout().flush()?;
            }
            Err(error) => {
                println!("error: {error:?}");
            }
        }
        Ok(())
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye! :D");
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        print!("\x1b[2J");
        if let Err(error) = self.draw_rows() {
            panic!("error: {error:?}");
        }
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                print!(
                    "Code: {code:?}, Modifiers: {modifiers:?}, Kind: {kind:?}, State: {state:?}\n"
                );

                match code {
                    Char('q') => self.should_exit = true,
                    _ => (),
                }
            }

            if self.should_exit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
