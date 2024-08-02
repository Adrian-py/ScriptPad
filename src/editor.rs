use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;
use terminal::{CursorPosition, Terminal, TerminalSize};
mod terminal;

pub struct Editor {
    cursor_position: CursorPosition,
    should_exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            cursor_position: CursorPosition { x: 2, y: 0 },
            should_exit: false,
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let editor_res = self.repl();
        Terminal::terminate().unwrap();
        editor_res.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
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
            Self::draw_greet_message("ScriptPad Editor -- version 1")?;
            Terminal::move_cursor_to(self.cursor_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_greet_message(message: &str) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;

        let msg_col_position: usize = terminal_size.height / 3;
        let msg_row_position: usize = (terminal_size.width - message.len()) / 2;
        Terminal::move_cursor_to(CursorPosition {
            x: msg_row_position,
            y: msg_col_position,
        })?;
        Terminal::print(message)?;

        Ok(())
    }

    fn draw_row_borders() -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::move_cursor_to(CursorPosition { x: 0, y: curr_row })?;
            Terminal::print("~ ")?;
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Error> {
        // If CTRL + q is pressed, then terminate
        if let Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_exit = true;
            return Ok(());
        }

        if let Key(KeyEvent { code, kind, .. }) = event {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Char(c) => Terminal::print(c)?,
                    KeyCode::Left => {
                        self.cursor_position.x = self.cursor_position.x.max(3) - 1;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::Right => {
                        self.cursor_position.x =
                            self.cursor_position.x.min(Terminal::size()?.width - 2) + 1;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::Down => {
                        self.cursor_position.y =
                            self.cursor_position.y.min(Terminal::size()?.height) + 1;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::Up => {
                        self.cursor_position.y = self.cursor_position.y.max(1) - 1;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::PageDown => {
                        self.cursor_position.y = Terminal::size()?.height;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::PageUp => {
                        self.cursor_position.y = 0;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::Home => {
                        self.cursor_position.y = 2;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    KeyCode::End => {
                        self.cursor_position.y = Terminal::size()?.width;
                        Terminal::move_cursor_to(self.cursor_position)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
