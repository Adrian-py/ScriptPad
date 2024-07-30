use crossterm::cursor::MoveTo;
use crossterm::event::KeyModifiers;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Error};

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_exit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let editor_res = self.repl();
        Self::terminate().unwrap();
        editor_res.unwrap();
    }

    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.handle_event(event);

            self.refresh_screen()?;

            if self.should_exit {
                break;
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        if self.should_exit {
            Self::clear_screen()?;
            println!("Goodbye! :D");
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        // If CTRL + q is pressed, then terminate
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_exit = true;
            return;
        }

        if let Key(KeyEvent {
            code: Char(key), ..
        }) = event
        {
            println!("Pressed key: {key}");
        }
    }
}
