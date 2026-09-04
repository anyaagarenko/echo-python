use std::path::Path;

use echo_python::{check_source, Diagnostic, RULE_NUMBERS_LIST_SORTED};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source).expect("lint")
}

#[test]
fn sorted_numbers_are_clean() {
    assert!(lint("x = [4911, 100500]\n").is_empty());
    assert!(lint("x = [-2, -1, 0, 3]\n").is_empty());
}

#[test]
fn unsorted_numbers_are_reported() {
    let diags = lint("x = [100500, 4911]\n");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, RULE_NUMBERS_LIST_SORTED);
}

#[test]
fn noqa_suppresses() {
    assert!(lint("x = [2, 1]  # noqa: echo-numbers-list-sorted\n").is_empty());
}

#[test]
fn other_noqa_does_not_suppress() {
    assert!(!lint("x = [2, 1]  # noqa: echo-words-list-sorted\n").is_empty());
}
