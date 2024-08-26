use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    pub fn saturating_add(&self, other: usize) -> usize {
        match self {
            GraphemeWidth::Full => other.saturating_add(2),
            GraphemeWidth::Half => other.saturating_add(1),
        }
    }
}

#[derive(Debug)]
pub struct TextGrapheme {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
}

impl TextGrapheme {
    pub fn new(grapheme: &str) -> Self {
        let mut grapheme_string = grapheme;
        let grapheme_width = grapheme_string.width();

        if grapheme_string == "\t" {
            grapheme_string = " ";
        }

        let mut replacement = None;
        if grapheme_width == 0 {
            replacement = Some('·');
        }

        Self {
            grapheme: grapheme_string.to_string(),
            rendered_width: if grapheme_width >= 2 {
                GraphemeWidth::Full
            } else {
                GraphemeWidth::Half
            },
            replacement,
        }
    }
}
