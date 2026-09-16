use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_CLASS_ATTRIBUTE_EMPTY_LINES, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

#[test]
fn consecutive_attributes_are_clean() {
    let source = "class S:\n    a = 1\n    b = 2\n";
    assert!(lint(source).is_empty());
}

#[test]
fn blank_between_attributes_is_reported() {
    let source = "class S:\n    a = 1\n\n    b = 2\n";
    let diags = lint(source);
    assert_eq!(1, diags.len());
    assert_eq!(RULE_CLASS_ATTRIBUTE_EMPTY_LINES, diags[0].code);
    assert_eq!(3, diags[0].row);
}

#[test]
fn blank_after_meta_is_clean() {
    let source = "class S:\n    class Meta:\n        x = 1\n\n    a = 1\n    b = 2\n";
    assert!(lint(source).is_empty());
}

#[test]
fn blank_between_methods_is_clean() {
    let source = "class S:\n    a = 1\n\n    def f(self):\n        pass\n    b = 2\n";
    assert!(lint(source).is_empty());
}

#[test]
fn blank_after_nested_class_is_clean() {
    let source = "class Widget:\n    class Config:\n        timeout = 1\n\n    name = \"\"\n    version = 1\n";
    assert!(lint(source).is_empty());
}

#[test]
fn dataclass_fields_are_clean() {
    let source = "@dataclass\nclass Point:\n    x: int\n    y: int\n";
    assert!(lint(source).is_empty());
}

#[test]
fn dataclass_blank_between_fields_is_reported() {
    let source = "@dataclass\nclass Point:\n    x: int\n\n    y: int\n";
    let diags = lint(source);
    assert_eq!(1, diags.len());
    assert_eq!(RULE_CLASS_ATTRIBUTE_EMPTY_LINES, diags[0].code);
    assert_eq!(4, diags[0].row);
}

#[test]
fn grouped_attributes_with_blank_is_reported() {
    let source = "class S:\n    a = 1\n    b = 2\n\n    c = 3\n";
    let diags = lint(source);
    assert_eq!(1, diags.len());
    assert_eq!(4, diags[0].row);
}
