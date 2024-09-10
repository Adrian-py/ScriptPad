use super::line::Line;
use std::{fs::read_to_string, io::Error};

pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Buffer {
    pub fn load(file_path: &str) -> Result<Self, Error> {
        let file_to_string = read_to_string(file_path)?;
        let mut lines: Vec<Line> = Vec::new();
        for line in file_to_string.lines() {
            lines.push(Line::from(line));
        }
        lines.push(Line::from(""));

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() == 0
    }

    pub fn insert(&mut self, inserted_char: char, insert_row: usize, line_insert_location: usize) {
        self.lines[insert_row].insert_char(inserted_char, line_insert_location);
    }

    pub fn remove(&mut self, remove_row: usize, line_remove_location: usize) {
        if line_remove_location == 0 {
            return; // TODO: Handle combining current line with line above when hitting backspace on the beginning of a line
        }
        self.lines[remove_row].remove_char(line_remove_location.saturating_sub(1));
    }
}
