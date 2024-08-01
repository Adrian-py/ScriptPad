use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::{stdout, Error, Write};
use terminal::Terminal;
mod terminal;

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_exit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let editor_res = self.repl();
        Terminal::terminate().unwrap();
        editor_res.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            Self::draw_row_borders()?;
            self.refresh_screen()?;
            if self.should_exit {
                break;
            }

            let event = read()?;
            self.handle_event(event);
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        if self.should_exit {
            Terminal::clear_screen()?;
            println!("Goodbye! :D");
        } else {
            Self::draw_row_borders()?;
        }

        Ok(())
    }

    fn draw_row_borders() -> Result<(), Error> {
        let (_width, height) = Terminal::get_size()?;
        for i in 0..=height {
            Terminal::move_cursor_to(0, i)?;
            stdout().write(b"~ ")?;
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
