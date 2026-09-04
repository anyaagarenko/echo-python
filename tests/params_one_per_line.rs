use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_PARAMS_ONE_PER_LINE, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn one_param_is_clean() {
    assert!(lint("def f(a):\n    pass\n").is_empty());
}

#[test]
fn two_params_same_line_are_reported() {
    let diags = lint("def f(a, b):\n    pass\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_PARAMS_ONE_PER_LINE, diags[0].code);
}

#[test]
fn two_params_one_per_line_are_clean() {
    assert!(lint("def f(\n    a,\n    b,\n):\n    pass\n").is_empty());
}

#[test]
fn self_with_one_param_is_clean() {
    assert!(lint("def f(self, a):\n    pass\n").is_empty());
}

#[test]
fn self_with_two_params_same_line_are_reported() {
    assert_eq!(
        RULE_PARAMS_ONE_PER_LINE,
        lint("def f(self, a, b):\n    pass\n")[0].code
    );
}

#[test]
fn noqa_suppresses() {
    assert!(lint("def f(a, b):  # noqa: ECHO005\n    pass\n").is_empty());
}
