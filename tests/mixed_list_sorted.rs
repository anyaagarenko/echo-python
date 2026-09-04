use std::path::Path;

use echo_python::{check_source, Diagnostic, RULE_MIXED_LIST_SORTED};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source).expect("lint")
}

#[test]
fn sorted_mixed_is_clean() {
    assert!(lint("x = [1, 2, \"a\", \"b\"]\n").is_empty());
    assert!(lint("x = [4911, 100500, \"a\"]\n").is_empty());
}

#[test]
fn word_before_number_is_reported() {
    let diags = lint("x = [\"a\", 1]\n");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, RULE_MIXED_LIST_SORTED);
}

#[test]
fn unsorted_numbers_in_mixed_are_reported() {
    assert_eq!(lint("x = [2, 1, \"a\"]\n")[0].code, RULE_MIXED_LIST_SORTED);
}

#[test]
fn unsorted_words_in_mixed_are_reported() {
    assert_eq!(
        lint("x = [1, \"b\", \"a\"]\n")[0].code,
        RULE_MIXED_LIST_SORTED
    );
}

#[test]
fn complex_expressions_are_skipped() {
    assert!(lint("x = [1 + 1, 0]\n").is_empty());
}

#[test]
fn noqa_suppresses() {
    assert!(lint("x = [\"a\", 1]  # noqa: echo-mixed-list-sorted\n").is_empty());
}
