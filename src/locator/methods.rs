use rustpython_parser::text_size::TextSize;

use super::Locator;

impl Locator {
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
}
