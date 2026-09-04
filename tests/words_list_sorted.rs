use std::path::Path;

use echo_python::{check_source, Diagnostic, RULE_WORDS_LIST_SORTED};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source).expect("lint")
}

#[test]
fn sorted_strings_are_clean() {
    assert!(lint("x = [\"a\", \"b\"]\n").is_empty());
}

#[test]
fn unsorted_strings_are_reported() {
    let diags = lint("x = [\"b\", \"a\"]\n");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, RULE_WORDS_LIST_SORTED);
}

#[test]
fn names_are_sorted_as_words() {
    assert!(lint("x = [a, b]\n").is_empty());
    assert_eq!(lint("x = [b, a]\n")[0].code, RULE_WORDS_LIST_SORTED);
}

#[test]
fn noqa_suppresses() {
    assert!(lint("x = [\"b\", \"a\"]  # noqa: echo-words-list-sorted\n").is_empty());
}
