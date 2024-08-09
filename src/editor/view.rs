use crate::editor::terminal::{CursorPosition, Terminal, TerminalSize};
use buffer::Buffer;
use std::io::Error;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        self.draw_rows()?;
        self.draw_greet_message()?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::move_cursor_to(CursorPosition { x: 0, y: curr_row })?;
            Terminal::print("~ ")?;
            if curr_row < self.buffer.lines.len() {
                Terminal::print(&self.buffer.lines[curr_row])?;
            }
        }

        Ok(())
    }

    fn draw_greet_message(&self) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        let message: &str = &format!("{NAME} editor -- version {VERSION}");

        #[allow(clippy::integer_division)]
        let msg_col_position: usize = terminal_size.height / 3;
        let msg_row_position: usize = (terminal_size.width - message.len()) / 2;

        Terminal::move_cursor_to(CursorPosition {
            x: msg_row_position,
            y: msg_col_position,
        })?;
        Terminal::print(message)?;

        Ok(())
    }
}
