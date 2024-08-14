use super::position::Position;
use crate::editor::{command::Direction, terminal::Terminal};

#[derive(Default)]
pub struct Caret {
    pub position: Position,
}

impl Caret {
    pub fn move_caret(&mut self, direction: &Direction) {
        let terminal_size = Terminal::size().unwrap_or_default();
        // To handle diectionally related keyboard events
        match direction {
            Direction::Up => self.position.row = self.position.row.saturating_sub(1),
            Direction::Down => self.position.row = self.position.row.saturating_add(1),
            Direction::Left => self.position.col = self.position.col.saturating_sub(1),
            Direction::Right => self.position.col = self.position.col.saturating_add(1),
            Direction::PageUp => self.position.row = 0,
            Direction::PageDown => self.position.row = terminal_size.height.saturating_sub(1),
            Direction::Home => self.position.col = 0,
            Direction::End => self.position.col = terminal_size.width.saturating_sub(1),
        }
    }
}
