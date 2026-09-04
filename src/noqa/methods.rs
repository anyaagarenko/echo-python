use super::NoqaIndex;

impl NoqaIndex {
    pub(crate) fn suppresses(&self, line: usize, code: &str) -> bool {
        self.lines
            .get(&line)
            .is_some_and(|codes| codes.is_empty() || codes.contains(code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_noqa_suppresses_all() {
        let index = NoqaIndex::from_source("x = [2, 1]  # noqa\n");
        assert!(index.suppresses(1, "echo-numbers-list-sorted"));
    }

    #[test]
    fn specific_noqa_is_case_insensitive() {
        let index = NoqaIndex::from_source("x = [2, 1]  # noqa: ECHO-NUMBERS-LIST-SORTED\n");
        assert!(index.suppresses(1, "echo-numbers-list-sorted"));
    }

    #[test]
    fn unrelated_comment_does_not_suppress() {
        let index = NoqaIndex::from_source("x = [2, 1]  # not a noqa\n");
        assert!(!index.suppresses(1, "echo-numbers-list-sorted"));
    }

    #[test]
    fn multiple_codes() {
        let index = NoqaIndex::from_source("x = [2, 1]  # noqa: echo-numbers-list-sorted, other\n");
        assert!(index.suppresses(1, "echo-numbers-list-sorted"));
    }

    #[test]
    fn other_code_does_not_suppress() {
        let index = NoqaIndex::from_source("x = [2, 1]  # noqa: echo-words-list-sorted\n");
        assert!(!index.suppresses(1, "echo-numbers-list-sorted"));
    }
}
