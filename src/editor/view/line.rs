use std::ops::Range;

pub struct Line {
    line_content: String,
}

impl Line {
    pub fn from(content: &str) -> Self {
        Self {
            line_content: content.to_string(),
        }
    }

    pub fn len(&self) -> usize {
        self.line_content.len()
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let range_start = range.start;
        let range_end = range.end.min(self.line_content.len());
        self.line_content
            .get(range_start..range_end)
            .unwrap_or_default()
            .to_string()
    }

    pub fn to_string(&self) -> &String {
        &self.line_content
    }
}
