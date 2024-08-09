use crate::editor::terminal::{CursorPosition, Terminal, TerminalSize};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
        Self::draw_rows()?;
        Self::draw_greet_message()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::move_cursor_to(CursorPosition { x: 0, y: curr_row })?;
            Terminal::print("~ ")?;
            if curr_row == 0 {
                Terminal::print("Hello, World!")?;
            }
        }

        Ok(())
    }

    fn draw_greet_message() -> Result<(), Error> {
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
