use std::path::Path;

use echo_python::{Diagnostic, RULE_MIXED_LIST_SORTED, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source).expect("lint")
}

#[test]
fn sorted_mixed_strings_are_clean() {
    assert!(lint("x = [1, 2, \"a\", \"b\"]\n").is_empty());
}

#[test]
fn sorted_mixed_with_large_numbers_are_clean() {
    assert!(lint("x = [4911, 100500, \"a\"]\n").is_empty());
}

#[test]
fn word_before_number_is_reported() {
    let diags = lint("x = [\"a\", 1]\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_MIXED_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_numbers_in_mixed_are_reported() {
    assert_eq!(RULE_MIXED_LIST_SORTED, lint("x = [2, 1, \"a\"]\n")[0].code);
}

#[test]
fn unsorted_words_in_mixed_are_reported() {
    assert_eq!(
        RULE_MIXED_LIST_SORTED,
        lint("x = [1, \"b\", \"a\"]\n")[0].code
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
