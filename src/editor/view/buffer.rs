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

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len() == 0
    }
}
