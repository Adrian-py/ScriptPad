use super::command::{Command, Direction};
use super::terminal::{Size, Terminal};
use buffer::Buffer;
use position::Position;

mod buffer;
mod line;
pub mod position;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    caret_position: Position,
    scroll_offset: Position,
    buffer: Buffer,
    needs_redraw: bool,
    terminal_size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            caret_position: Position::default(),
            scroll_offset: Position::default(),
            buffer: Buffer::default(),
            needs_redraw: true,
            terminal_size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn load(&mut self, file_path: &str) {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Move(direction) => self.move_caret(direction),
            Command::Resize(new_size) => self.terminal_resize(new_size),
            _ => {}
        }
    }

    pub fn terminal_resize(&mut self, new_size: Size) {
        self.terminal_size = new_size;
        self.needs_redraw = true;
    }

    pub fn move_caret(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.caret_position.row = self.caret_position.row.saturating_sub(1),
            Direction::Down => self.caret_position.row = self.caret_position.row.saturating_add(1),
            Direction::Left => self.caret_position.col = self.caret_position.col.saturating_sub(1),
            Direction::Right => self.caret_position.col = self.caret_position.col.saturating_add(1),
            Direction::PageUp => self.caret_position.row = 0,
            Direction::PageDown => {
                self.caret_position.row = self.terminal_size.height.saturating_sub(1)
            }
            Direction::Home => self.caret_position.col = 0,
            Direction::End => self.caret_position.col = self.terminal_size.width.saturating_sub(1),
        }
        self.needs_redraw = true;
        self.adjust_screen_to_offset();
    }

    pub fn get_position(&self) -> Position {
        Position {
            row: self
                .caret_position
                .row
                .saturating_sub(self.scroll_offset.row),
            col: self
                .caret_position
                .col
                .saturating_sub(self.scroll_offset.col),
        }
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

    fn render_line(&self, row_index: usize, line_content: &str) {
        let modified_line_content = &line_content[self.scroll_offset.col.min(line_content.len())..];
        let print_res = Terminal::print_row(row_index, modified_line_content);
        debug_assert!(print_res.is_ok(), "Failed to print row!");
    }

    fn render_welcome(&self) {
        let terminal_size: Size = Terminal::size().unwrap_or_default();

        for curr_row in 0..terminal_size.height {
            #[allow(clippy::integer_division)]
            if curr_row == terminal_size.height / 3 {
                self.draw_greet_message(curr_row);
                continue;
            }
            self.render_line(curr_row, "~");
        }
    }

    fn render_buffer(&self) {
        for curr_row in 0..self.terminal_size.height {
            if let Some(curr_line) = self
                .buffer
                .lines
                .get(curr_row.saturating_add(self.scroll_offset.row))
            {
                let left = self.scroll_offset.col;
                let right = self
                    .scroll_offset
                    .col
                    .saturating_add(self.terminal_size.width);
                self.render_line(curr_row, &curr_line.get(left..right));
            } else {
                self.render_line(curr_row, "~");
            }
        }
    }

    fn draw_greet_message(&self, row_index: usize) {
        let mut message: String = format!("{NAME} editor -- version {VERSION}");
        #[allow(clippy::arithmetic_side_effects, clippy::integer_division)]
        let spaces: String = " ".repeat((self.terminal_size.width - message.len()) / 2 - 1);
        message = format!("~{spaces}{message}\r\n");

        if message.len() > self.terminal_size.width {
            message = message[0..self.terminal_size.width].to_string();
        }
        self.render_line(row_index, &message);
    }

    fn adjust_screen_to_offset(&mut self) {
        // Horizontal
        if self.caret_position.col < self.scroll_offset.col {
            self.scroll_offset.col = self.caret_position.col;
            self.needs_redraw = true;
        } else if self.caret_position.col
            >= self
                .scroll_offset
                .col
                .saturating_add(self.terminal_size.width)
        {
            self.scroll_offset.col = self
                .caret_position
                .col
                .saturating_sub(self.terminal_size.width)
                .saturating_add(1);
            self.needs_redraw = true;
        }

        // Vertical offset
        if self.caret_position.row < self.scroll_offset.row {
            self.scroll_offset.row = self.caret_position.row;
            self.needs_redraw = true;
        } else if self.caret_position.row
            >= self
                .scroll_offset
                .row
                .saturating_add(self.terminal_size.height)
        {
            self.scroll_offset.row = self
                .caret_position
                .row
                .saturating_sub(self.terminal_size.height)
                .saturating_add(1);
            self.needs_redraw = true;
        }
    }
}
