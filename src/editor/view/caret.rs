use super::{buffer::Buffer, position::Position};
use crate::editor::{command::Direction, terminal::Terminal};

#[derive(Default)]
pub struct Caret {
    pub position: Position,
}

impl Caret {
    pub fn move_caret(&mut self, direction: &Direction, buffer: &Buffer) {
        let Position { mut row, mut col } = self.position;
        let lines = &buffer.lines;

        if lines.len() == 0 {
            return;
        }

        // To handle diectionally related keyboard events
        match direction {
            Direction::Up => {
                row = row.saturating_sub(1);
                if col > lines[row].len() {
                    col = lines[row].len();
                }
            }
            Direction::Down => {
                row = row.saturating_add(1).min(lines.len().saturating_sub(1));
                if col > lines[row].len() {
                    col = lines[row].len();
                }
            }
            Direction::Left => {
                if col == 0 && row > 0 {
                    row = row.saturating_sub(1);
                    col = lines[row].len();
                } else {
                    col = col.saturating_sub(1);
                }
            }
            Direction::Right => {
                if col == lines[row].len() && row.saturating_add(1) < lines.len() {
                    col = 0;
                    row = row.saturating_add(1);
                } else {
                    col = col.saturating_add(1).min(lines[row].len());
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
