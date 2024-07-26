use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_exit: false }
    }

    // pub fn draw_rows(&self) {
    //     if let Ok((col, row)) = crossterm::terminal::size()? {}
    // }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye! :D");
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        // self.draw_rows();
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
