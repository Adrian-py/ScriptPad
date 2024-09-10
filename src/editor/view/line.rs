use super::Position;
use std::ops::Range;
use text_grapheme::TextGrapheme;

use unicode_segmentation::UnicodeSegmentation;

mod text_grapheme;
#[derive(Debug)]
pub struct Line {
    pub line_content: Vec<TextGrapheme>,
}

impl Line {
    pub fn from(content: &str) -> Self {
        let mut content_graphemes: Vec<TextGrapheme> = Vec::new();
        for grapheme in content.graphemes(true) {
            content_graphemes.push(TextGrapheme::new(grapheme));
        }

        Self {
            line_content: content_graphemes,
        }
    }

    pub fn len(&self) -> usize {
        self.line_content.len()
    }

    pub fn get_total_width(&self) -> usize {
        let mut total_length: usize = 0;
        for grapheme in &self.line_content {
            total_length = grapheme.rendered_width.saturating_add(total_length);
        }

        total_length
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        let mut result_string: String = String::new();
        if range.start >= range.end {
            return result_string;
        }

        let mut screen_position = 0;
        let range_start = range.start;
        let range_end: usize = range.end.min(self.get_total_width());

        for grapheme in &self.line_content {
            let grapheme_end = grapheme.rendered_width.saturating_add(screen_position);

            if screen_position >= range_end {
                break;
            }

            if grapheme_end > range_start {
                if (screen_position < range_start && grapheme_end >= range_start)
                    || (screen_position < range_end && grapheme_end > range_end)
                {
                    result_string.push_str("...");
                } else if let Some(replacement) = grapheme.replacement {
                    result_string.push(replacement);
                } else {
                    result_string.push_str(&grapheme.grapheme);
                }
            }
            screen_position = grapheme_end;
        }

        result_string
    }

    pub fn get_nth_location(&self, col: usize) -> usize {
        let mut total_prev_width: usize = 0;
        for t in 0..col.min(self.line_content.len()) {
            total_prev_width = self.line_content[t]
                .rendered_width
                .saturating_add(total_prev_width);
        }

        total_prev_width
    }

    pub fn insert_char(&mut self, inserted_char: char, line_insert_location: usize) {
        let new_grapheme = TextGrapheme::new(&format!("{inserted_char}"));
        self.line_content.insert(line_insert_location, new_grapheme);
    }

    pub fn remove_char(&mut self, line_remove_location: usize) {
        self.line_content.remove(line_remove_location);
    }
}
