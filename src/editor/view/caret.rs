use super::{buffer::Buffer, line::Line, position::Position};
use crate::editor::command::Direction;

#[derive(Default)]
pub struct Caret {
    pub position: Position,
    pub line_location: usize,
}

impl Caret {
    pub fn move_caret(&mut self, direction: &Direction, buffer: &Buffer) {
        let Position { mut row, mut col } = self.position;
        let lines: &Vec<Line> = &buffer.lines;

        // To handle diectionally related keyboard events
        match direction {
            Direction::Up => {
                row = row.saturating_sub(1);
                self.line_location = self.line_location.min(lines[row].len());
                col = lines[row].get_nth_location(self.line_location);
            }
            Direction::Down => {
                row = row.saturating_add(1).min(lines.len().saturating_sub(1));
                self.line_location = self.line_location.min(lines[row].len());
                col = lines[row].get_nth_location(self.line_location);
            }
            Direction::Left => {
                if self.line_location == 0 && row > 0 {
                    row = row.saturating_sub(1);
                    col = lines[row].get_total_width();
                    self.line_location = lines[row].len();
                } else {
                    self.line_location = self.line_location.saturating_sub(1);
                    col = lines[row].get_nth_location(self.line_location);
                }
            }
            Direction::Right => {
                if self.line_location == lines[row].len() {
                    row = row.saturating_add(1).min(lines.len().saturating_sub(1));
                    col = 0;
                    self.line_location = 0;
                } else {
                    self.line_location = self.line_location.saturating_add(1);
                    col = lines[row].get_nth_location(self.line_location);
                }
            }
            Direction::PageUp => row = 0,
            Direction::PageDown => row = lines.len().saturating_sub(1),
            Direction::Home => col = 0,
            Direction::End => col = lines[row].len(),
        }

        self.position = Position { row, col };
    }
}
