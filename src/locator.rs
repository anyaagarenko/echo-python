use rustpython_parser::text_size::TextSize;

pub struct Locator {
    line_starts: Vec<TextSize>,
}

impl Locator {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![TextSize::from(0u32)];
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(TextSize::try_from(idx + 1).expect("source too large"));
            }
        }
        Self { line_starts }
    }

    pub fn line_index(&self, offset: TextSize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    }

    pub fn column_index(&self, offset: TextSize) -> usize {
        let line = self.line_index(offset);
        let line_start = self.line_starts[line - 1];
        (offset - line_start).to_usize() + 1
    }
}
