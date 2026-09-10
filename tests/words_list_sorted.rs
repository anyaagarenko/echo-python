use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_WORDS_LIST_SORTED, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn sorted_strings_are_clean() {
    assert!(lint("x = [\"a\", \"b\"]\n").is_empty());
}

#[test]
fn unsorted_strings_are_reported() {
    let diags = lint("x = [\"b\", \"a\"]\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_WORDS_LIST_SORTED, diags[0].code);
}

#[test]
fn sorted_names_are_clean() {
    assert!(lint("x = [a, b]\n").is_empty());
}

#[test]
fn unsorted_names_are_reported() {
    assert_eq!(RULE_WORDS_LIST_SORTED, lint("x = [b, a]\n")[0].code);
}

#[test]
fn noqa_suppresses() {
    assert!(lint("x = [\"b\", \"a\"]  # noqa: ECHO004\n").is_empty());
}
