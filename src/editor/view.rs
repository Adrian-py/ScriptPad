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
    pub fn load(&mut self, file_path: &str) {
        if let Ok(file_string) = std::fs::read_to_string(file_path) {
            self.buffer.add_lines(&file_string);
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            self.render_welcome()?;
        } else {
            self.render_buffer()?;
        }

        Ok(())
    }

    fn render_welcome(&self) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if curr_row == terminal_size.height / 3 {
                self.draw_greet_message()?;
                continue;
            }

            let mut line_contents = String::from("~ ");
            if curr_row.saturating_add(1) < terminal_size.height {
                line_contents.push_str("\r\n");
            }
            Terminal::print(&line_contents)?;
        }

        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;
        for curr_row in 0..terminal_size.height {
            Terminal::clear_line()?;
            if let Some(curr_line) = self.buffer.lines.get(curr_row) {
                Terminal::print(curr_line)?;
            } else {
                Terminal::print("~")?;
            }
            if curr_row < terminal_size.height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_greet_message(&self) -> Result<(), Error> {
        let terminal_size: TerminalSize = Terminal::size()?;

        let mut message: String = format!("{NAME} editor -- version {VERSION}");
        let spaces: String = " ".repeat((terminal_size.width - message.len()) / 2 - 1);
        message = format!("~{spaces}{message}\r\n");
        Terminal::print(&message)?;

        Ok(())
    }
}
