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

    pub fn merge_next_line(&mut self, current_row_index: usize) {
        // Merging line of index 'current_row_index' with the next line
        if current_row_index.saturating_add(1) >= self.lines.len() {
            return;
        }

        let (current_row, next_row) = {
            let (left, right) = self.lines.split_at_mut(current_row_index.saturating_add(1));
            (&mut left[current_row_index], &mut right[0])
        };

        current_row.append(&mut next_row.line_content);
        self.lines.remove(current_row_index.saturating_add(1));
    }

    /**
     * Command Operations
     */
    pub fn insert(&mut self, inserted_char: char, insert_row: usize, line_insert_location: usize) {
        self.lines[insert_row].insert(inserted_char, line_insert_location);
    }

    pub fn remove(&mut self, remove_row: usize, line_remove_location: usize) {
        // Merge current line with previous line, if caret is currently in the beginning of a line
        if line_remove_location == 0 {
            if remove_row > 0 {
                self.merge_next_line(remove_row.saturating_sub(1));
            }
            return;
        }

        self.lines[remove_row].remove(line_remove_location.saturating_sub(1));
    }

    pub fn delete(&mut self, remove_row: usize, line_delete_location: usize) {
        // Merge previous line with current line, if caret is at the end of a line
        if line_delete_location == self.lines[remove_row].len() {
            if remove_row.saturating_add(1) < self.lines.len() {
                self.merge_next_line(remove_row);
            }
            return;
        }
        self.lines[remove_row].delete(line_delete_location);
    }
}
