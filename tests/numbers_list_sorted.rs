use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_NUMBERS_LIST_SORTED, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn sorted_positive_numbers_are_clean() {
    assert!(lint("x = [4911, 100500]\n").is_empty());
}

#[test]
fn sorted_signed_numbers_are_clean() {
    assert!(lint("x = [-2, -1, 0, 3]\n").is_empty());
}

#[test]
fn unsorted_numbers_are_reported() {
    let diags = lint("x = [100500, 4911]\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}

#[test]
fn noqa_suppresses() {
    assert!(lint("x = [2, 1]  # noqa: ECHO003\n").is_empty());
}

#[test]
fn other_noqa_does_not_suppress() {
    assert!(!lint("x = [2, 1]  # noqa: ECHO004\n").is_empty());
}
