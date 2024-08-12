use crate::editor::terminal::{Size, Terminal};
use buffer::Buffer;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    terminal_size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            terminal_size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn load(&mut self, file_path: &str) {
        if let Ok(file_string) = std::fs::read_to_string(file_path) {
            self.buffer.add_lines(&file_string);
        }
    }

    pub fn window_resize(&mut self, new_size: Size) {
        self.terminal_size = new_size;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        if self.buffer.is_empty() {
            self.render_welcome();
        } else {
            self.render_buffer();
        }
        self.needs_redraw = false;
    }

    fn render_line(row_index: usize, line_content: &str) {
        let print_res = Terminal::print_row(row_index, line_content);
        debug_assert!(print_res.is_ok(), "Failed to print row!");
    }

    fn render_welcome(&self) {
        let terminal_size: Size = Terminal::size().unwrap_or_default();

        for curr_row in 0..terminal_size.height {
            #[allow(clippy::integer_division)]
            if curr_row == terminal_size.height / 3 {
                Self::draw_greet_message(curr_row);
                continue;
            }
            Self::render_line(curr_row, "~");
        }
    }

    fn render_buffer(&self) {
        let terminal_size: Size = Terminal::size().unwrap_or_default();

        for curr_row in 0..terminal_size.height {
            if let Some(curr_line) = self.buffer.lines.get(curr_row) {
                let mut truncated_line: &str = curr_line;
                if curr_line.len() > terminal_size.width {
                    truncated_line = &curr_line[0..terminal_size.width];
                }

                Self::render_line(curr_row, truncated_line);
            } else {
                Self::render_line(curr_row, "~");
            }
        }
    }

    fn draw_greet_message(row_index: usize) {
        let terminal_size: Size = Terminal::size().unwrap_or_default();

        let mut message: String = format!("{NAME} editor -- version {VERSION}");
        #[allow(clippy::arithmetic_side_effects, clippy::integer_division)]
        let spaces: String = " ".repeat((terminal_size.width - message.len()) / 2 - 1);
        message = format!("~{spaces}{message}\r\n");

        if message.len() > terminal_size.width {
            message = message[0..terminal_size.width].to_string();
        }
        Self::render_line(row_index, &message);
    }
}
