use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{CursorPosition, Terminal, TerminalSize};
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
            self.handle_event(event)?;
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(CursorPosition { x: 0, y: 0 })?;
        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! :D")?;
        } else {
            Self::draw_row_borders()?;
            Terminal::move_cursor_to(CursorPosition { x: 2, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_row_borders() -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::get_size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::clear_line()?;
            Terminal::print("~ ")?;

            if curr_row + 1 < terminal_size.height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        // If CTRL + q is pressed, then terminate
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_exit = true;
            return Ok(());
        }

        if let Key(KeyEvent {
            code: Char(key), ..
        }) = event
        {
            Terminal::print(format!("Pressed: {}", key).as_str())?;
        }
        Ok(())
    }
}
