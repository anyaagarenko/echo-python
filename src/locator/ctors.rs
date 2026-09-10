use rustpython_parser::text_size::TextSize;

use super::Locator;

impl Locator {
    pub(crate) fn new(source: &str) -> Self {
        let mut line_starts = vec![TextSize::from(0u32)];
        for (idx, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(TextSize::try_from(idx + 1).expect("source too large"));
            }
        }
        Self { line_starts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_source_has_one_line_start() {
        let locator = Locator::new("");
        assert_eq!(1, locator.line_starts.len());
        assert_eq!(TextSize::from(0), locator.line_starts[0]);
    }

    #[test]
    fn tracks_newline_offsets() {
        let locator = Locator::new("ab\ncd\n");
        assert_eq!(3, locator.line_starts.len());
        assert_eq!(TextSize::from(0), locator.line_starts[0]);
        assert_eq!(TextSize::from(3), locator.line_starts[1]);
        assert_eq!(TextSize::from(6), locator.line_starts[2]);
    }
}
