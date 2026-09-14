use rustpython_parser::ast::Ranged;
use rustpython_parser::text_size::TextSize;

use super::Locator;

impl Locator<'_> {
    pub(crate) fn text(&self, node: &impl Ranged) -> &str {
        self.range(node.start(), node.end())
    }

    pub(crate) fn range(&self, start: TextSize, end: TextSize) -> &str {
        &self.source[start.to_usize()..end.to_usize()]
    }

    pub(crate) fn line_index(&self, offset: TextSize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        }
    }

    pub(crate) fn column_index(&self, offset: TextSize) -> usize {
        let line = self.line_index(offset);
        let line_start = self.line_starts[line - 1];
        (offset - line_start).to_usize() + 1
    }

    pub(crate) fn line_is_blank(&self, line: usize) -> bool {
        self.line_text(line).trim().is_empty()
    }

    pub(crate) fn line_start_offset(&self, line: usize) -> usize {
        self.line_starts[line - 1].to_usize()
    }

    fn line_text(&self, line: usize) -> &str {
        let start = self.line_start_offset(line);
        let end = self
            .line_starts
            .get(line)
            .map_or(self.source.len(), TextSize::to_usize);
        self.source[start..end].trim_end_matches(['\r', '\n'])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_index_at_file_start() {
        let locator = Locator::new("ab\ncd\n");
        assert_eq!(1, locator.line_index(TextSize::from(0)));
    }

    #[test]
    fn line_index_after_newline() {
        let locator = Locator::new("ab\ncd\n");
        assert_eq!(2, locator.line_index(TextSize::from(3)));
    }

    #[test]
    fn column_index_on_first_line() {
        let locator = Locator::new("x = [2, 1]\n");
        assert_eq!(5, locator.column_index(TextSize::from(4)));
    }

    #[test]
    fn column_index_on_second_line() {
        let locator = Locator::new("a\nb\n");
        assert_eq!(1, locator.column_index(TextSize::from(2)));
    }

    #[test]
    fn blank_line_with_spaces_is_blank() {
        let locator = Locator::new("a\n  \nb\n");
        assert!(locator.line_is_blank(2));
        assert!(!locator.line_is_blank(1));
    }

    #[test]
    fn range_returns_slice() {
        let locator = Locator::new("abcde");
        assert_eq!("bcd", locator.range(TextSize::from(1), TextSize::from(4)));
    }
}
