use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_EMPTY_LINES, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn solid_method_is_clean() {
    assert!(lint("def f():\n    a = 1\n    return a\n").is_empty());
}

#[test]
fn blank_inside_method_is_reported() {
    let diags = lint("def f():\n    a = 1\n\n    return a\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_EMPTY_LINES, diags[0].code);
    assert_eq!(3, diags[0].row);
}

#[test]
fn blank_after_header_is_reported() {
    assert_eq!(RULE_EMPTY_LINES, lint("def f():\n\n    return 1\n")[0].code);
}

#[test]
fn blank_between_methods_is_clean() {
    let source =
        "class C:\n    def f(self):\n        return 1\n\n    def g(self):\n        return 2\n";
    assert!(lint(source).is_empty());
}

#[test]
fn async_blank_is_reported() {
    assert_eq!(
        RULE_EMPTY_LINES,
        lint("async def f():\n    a = 1\n\n    return a\n")[0].code
    );
}

#[test]
fn nested_blank_is_reported_once() {
    let source = "def f():\n    def g():\n        a = 1\n\n        return a\n    return g\n";
    assert_eq!(1, lint(source).len());
}

#[test]
fn noqa_on_other_line_does_not_suppress() {
    assert_eq!(
        1,
        lint("def f():\n    a = 1  # noqa: ECHO007\n\n    return a\n").len()
    );
}
