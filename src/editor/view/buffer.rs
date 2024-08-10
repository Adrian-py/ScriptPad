pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 0
    }

    pub fn add_lines(&mut self, full_text: &str) {
        for line in full_text.split("\r\n") {
            self.lines.push(line.to_string());
        }
    }
}
