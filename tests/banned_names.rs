use std::path::{Path, PathBuf};

use echo_python::{CheckOptions, Diagnostic, RULE_BANNED_NAMES, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

fn lint_project(pyproject: &str, source: &str) -> Vec<Diagnostic> {
    let root = tempfile_dir("echo-python-echo006");
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
fn assignment_msg_is_reported() {
    let diags = lint("msg = 1\n");

    assert_eq!(1, diags.len());
    assert_eq!(RULE_BANNED_NAMES, diags[0].code);
}

#[test]
fn other_assignment_is_clean() {
    assert!(lint("error = 1\n").is_empty());
}

#[test]
fn except_as_msg_is_reported() {
    let diags = lint("try:\n    pass\nexcept Exception as msg:\n    pass\n");

    assert_eq!(RULE_BANNED_NAMES, diags[0].code);
}

#[test]
fn except_as_error_is_clean() {
    assert!(lint("try:\n    pass\nexcept Exception as error:\n    pass\n").is_empty());
}

#[test]
fn parameter_msg_is_reported() {
    assert_eq!(RULE_BANNED_NAMES, lint("def f(msg):\n    pass\n")[0].code);
}

#[test]
fn for_target_msg_is_reported() {
    assert_eq!(
        RULE_BANNED_NAMES,
        lint("for msg in xs:\n    pass\n")[0].code
    );
}

#[test]
fn with_as_msg_is_reported() {
    assert_eq!(
        RULE_BANNED_NAMES,
        lint("with open(\"a\") as msg:\n    pass\n")[0].code
    );
}

#[test]
fn walrus_msg_is_reported() {
    assert_eq!(
        RULE_BANNED_NAMES,
        lint("if (msg := 1):\n    pass\n")[0].code
    );
}

#[test]
fn import_as_msg_is_reported() {
    assert_eq!(RULE_BANNED_NAMES, lint("import os as msg\n")[0].code);
}

#[test]
fn reading_msg_is_clean() {
    assert!(lint("print(msg)\n").is_empty());
}

#[test]
fn noqa_suppresses() {
    assert!(lint("msg = 1  # noqa: ECHO006\n").is_empty());
}

#[test]
fn custom_names_from_pyproject() {
    let pyproject = "[tool.echo-python.echo006]\nnames = [\"msg\", \"err\"]\n";

    let diags = lint_project(pyproject, "err = 1\n");

    assert_eq!(RULE_BANNED_NAMES, diags[0].code);
}

#[test]
fn names_replace_default() {
    let pyproject = "[tool.echo-python.echo006]\nnames = [\"err\"]\n";

    assert!(lint_project(pyproject, "msg = 1\n").is_empty());
}

#[test]
fn match_as_msg_is_reported() {
    let source = "match value:\n    case msg:\n        pass\n";

    assert_eq!(RULE_BANNED_NAMES, lint(source)[0].code);
}
