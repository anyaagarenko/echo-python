use std::collections::HashMap;

use super::NoqaIndex;
use super::parse::parse_noqa;

impl NoqaIndex {
    pub(crate) fn from_source(source: &str) -> Self {
        let mut lines = HashMap::new();
        for (idx, line) in source.lines().enumerate() {
            if let Some(codes) = parse_noqa(line) {
                lines.insert(idx + 1, codes);
            }
        }
        Self { lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_bare_noqa() {
        let index = NoqaIndex::from_source("x = [2, 1]  # noqa\n");
        assert!(index.lines.contains_key(&1));
        assert!(index.lines[&1].is_empty());
    }

    #[test]
    fn indexes_specific_codes() {
        let index = NoqaIndex::from_source("x = 1  # noqa: echo-numbers-list-sorted, other\n");
        assert!(index.lines[&1].contains("echo-numbers-list-sorted"));
        assert!(index.lines[&1].contains("other"));
    }

    #[test]
    fn ignores_unrelated_comments() {
        let index = NoqaIndex::from_source("x = 1  # not a noqa\n");
        assert!(index.lines.is_empty());
    }
}
