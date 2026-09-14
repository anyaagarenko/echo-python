use std::path::{Path, PathBuf};

use echo_python::{CheckOptions, Diagnostic, RULE_EMPTY_LINES, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

fn lint_project(pyproject: &str, source: &str) -> Vec<Diagnostic> {
    let root = tempfile_dir("echo-python-echo007");
    std::fs::write(root.join("pyproject.toml"), pyproject).unwrap();
    let file = root.join("t.py");
    std::fs::write(&file, source).unwrap();
    let diags = check_source(&file, source, &CheckOptions::default()).expect("lint");
    let _ = std::fs::remove_dir_all(&root);
    diags
}

fn tempfile_dir(prefix: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
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
fn blank_inside_method_message_shows_name() {
    assert_eq!(
        "empty line inside `f`",
        lint("def f():\n    a = 1\n\n    return a\n")[0].message
    );
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

#[test]
fn test_prefixed_function_is_skipped() {
    assert!(lint("def test_f():\n    a = 1\n\n    return a\n").is_empty());
}

#[test]
fn helper_in_any_file_is_still_checked() {
    assert_eq!(1, lint("def helper():\n    a = 1\n\n    return a\n").len());
}

#[test]
fn exclude_tests_false_from_pyproject() {
    let pyproject = "[tool.echo-python.echo007]\nexclude_tests = false\n";

    assert_eq!(
        RULE_EMPTY_LINES,
        lint_project(pyproject, "def test_f():\n    a = 1\n\n    return a\n")[0].code
    );
}
