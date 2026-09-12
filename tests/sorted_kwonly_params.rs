use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_SORTED_KWONLY_PARAMS, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn sorted_kwonly_are_clean() {
    assert!(lint("def f(*, a, b):\n    pass\n").is_empty());
}

#[test]
fn unsorted_kwonly_are_reported() {
    let diags = lint("def f(*, b, a):\n    pass\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_SORTED_KWONLY_PARAMS, diags[0].code);
}

#[test]
fn positionals_are_ignored() {
    assert!(lint("def f(b, a):\n    pass\n").is_empty());
}

#[test]
fn positionals_with_unsorted_kwonly_are_reported() {
    assert_eq!(
        RULE_SORTED_KWONLY_PARAMS,
        lint("def f(z, y, *, b, a):\n    pass\n")[0].code
    );
}

#[test]
fn noqa_suppresses() {
    assert!(lint("def f(*, b, a):  # noqa: ECHO006\n    pass\n").is_empty());
}
