use rustpython_parser::ast::{self, Ranged};

use crate::RULE_CLASS_ATTRIBUTE_EMPTY_LINES;
use crate::common::quote;
use crate::common::report::report_at;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub(crate) fn check_class_def(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    stmt: &ast::StmtClassDef,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut previous: Option<&ast::Stmt> = None;
    for item in &stmt.body {
        if is_class_attribute(item) {
            if let Some(prev) = previous {
                report_blanks_between(
                    locator,
                    noqa,
                    path,
                    stmt.name.as_str(),
                    prev,
                    item,
                    diagnostics,
                );
            }
            previous = Some(item);
            continue;
        }
        previous = None;
    }
}

const fn is_class_attribute(stmt: &ast::Stmt) -> bool {
    matches!(stmt, ast::Stmt::Assign(_) | ast::Stmt::AnnAssign(_))
}

fn report_blanks_between(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    class_name: &str,
    previous: &ast::Stmt,
    next: &ast::Stmt,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let message = format!(
        "empty line between class attributes in {}",
        quote::tick(class_name)
    );
    let after_previous = locator.line_index(previous.end());
    let before_next = locator.line_index(next.start());
    for line in (after_previous + 1)..before_next {
        if !locator.line_is_blank(line) {
            continue;
        }
        report_at(
            noqa,
            path,
            line,
            1,
            RULE_CLASS_ATTRIBUTE_EMPTY_LINES,
            &message,
            diagnostics,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;
    use std::path::Path;

    fn lint(source: &str) -> Vec<Diagnostic> {
        let module = Suite::parse(source, "<test>").expect("parse");
        let locator = Locator::new(source);
        let noqa = NoqaIndex::from_source(source);
        let mut diagnostics = Vec::new();
        for stmt in module {
            lint_stmt(&locator, &noqa, &stmt, &mut diagnostics);
        }
        diagnostics
    }

    fn lint_stmt(
        locator: &Locator,
        noqa: &NoqaIndex,
        stmt: &ast::Stmt,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match stmt {
            ast::Stmt::ClassDef(node) => {
                check_class_def(locator, noqa, Path::new("t.py"), node, diagnostics);
                for item in &node.body {
                    lint_stmt(locator, noqa, item, diagnostics);
                }
            }
            ast::Stmt::FunctionDef(node) => {
                for item in &node.body {
                    lint_stmt(locator, noqa, item, diagnostics);
                }
            }
            _ => {}
        }
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
    fn blank_between_methods_only_is_clean() {
        let source = "class S:\n    def f(self):\n        pass\n\n    def g(self):\n        pass\n";
        assert!(lint(source).is_empty());
    }

    #[test]
    fn nested_class_attributes_are_checked() {
        let source = "class S:\n    class Meta:\n        a = 1\n\n        b = 2\n";
        assert_eq!(1, lint(source).len());
    }

    #[test]
    fn annotated_attributes_are_checked() {
        let source = "class S:\n    a: int = 1\n\n    b: int = 2\n";
        assert_eq!(1, lint(source).len());
    }

    #[test]
    fn message_shows_class_name() {
        let source = "class Widget:\n    a = 1\n\n    b = 2\n";
        assert_eq!(
            "empty line between class attributes in `Widget`",
            lint(source)[0].message
        );
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
    fn dataclass_fields_with_defaults_are_checked() {
        let source = "@dataclass\nclass Box:\n    w: int = 1\n\n    h: int = 2\n";
        assert_eq!(1, lint(source).len());
    }
}
