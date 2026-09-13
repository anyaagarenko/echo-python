use rustpython_parser::ast::{self, Ranged};
use rustpython_parser::text_size::TextSize;

use crate::RULE_EMPTY_LINES;
use crate::common::report::report_at;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

pub(crate) fn check_function_def(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    stmt: &ast::StmtFunctionDef,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if settings.echo007.skips_function(stmt.name.as_str()) {
        return;
    }
    check_body(locator, noqa, path, &stmt.body, diagnostics);
}

pub(crate) fn check_async_function_def(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    stmt: &ast::StmtAsyncFunctionDef,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if settings.echo007.skips_function(stmt.name.as_str()) {
        return;
    }
    check_body(locator, noqa, path, &stmt.body, diagnostics);
}

fn check_body(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    body: &[ast::Stmt],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(first) = body.first() else {
        return;
    };
    let Some(last) = body.last() else {
        return;
    };
    let nested = nested_ranges(body);
    let start_line = suite_start_line(locator, first);
    let end_line = locator.line_index(last.end());
    for line in start_line..=end_line {
        if !locator.line_is_blank(line) || line_in_nested(line, locator, &nested) {
            continue;
        }
        report_at(
            noqa,
            path,
            line,
            1,
            RULE_EMPTY_LINES,
            "empty line inside method",
            diagnostics,
        );
    }
}

fn suite_start_line(locator: &Locator, first: &ast::Stmt) -> usize {
    let mut line = locator.line_index(first.start());
    while line > 1 && locator.line_is_blank(line - 1) {
        line -= 1;
    }
    line
}

fn nested_ranges(body: &[ast::Stmt]) -> Vec<(TextSize, TextSize)> {
    let mut ranges = Vec::new();
    collect_nested(body, &mut ranges);
    ranges
}

fn collect_nested(stmts: &[ast::Stmt], ranges: &mut Vec<(TextSize, TextSize)>) {
    for stmt in stmts {
        match stmt {
            ast::Stmt::FunctionDef(node) => ranges.push((node.start(), node.end())),
            ast::Stmt::AsyncFunctionDef(node) => ranges.push((node.start(), node.end())),
            ast::Stmt::ClassDef(node) => collect_nested(&node.body, ranges),
            ast::Stmt::For(node) => {
                collect_nested(&node.body, ranges);
                collect_nested(&node.orelse, ranges);
            }
            ast::Stmt::AsyncFor(node) => {
                collect_nested(&node.body, ranges);
                collect_nested(&node.orelse, ranges);
            }
            ast::Stmt::While(node) => {
                collect_nested(&node.body, ranges);
                collect_nested(&node.orelse, ranges);
            }
            ast::Stmt::If(node) => collect_if(node, ranges),
            ast::Stmt::With(node) => collect_nested(&node.body, ranges),
            ast::Stmt::AsyncWith(node) => collect_nested(&node.body, ranges),
            ast::Stmt::Try(node) => collect_try(
                &node.body,
                &node.handlers,
                &node.orelse,
                &node.finalbody,
                ranges,
            ),
            ast::Stmt::TryStar(node) => collect_try(
                &node.body,
                &node.handlers,
                &node.orelse,
                &node.finalbody,
                ranges,
            ),
            ast::Stmt::Match(node) => {
                for case in &node.cases {
                    collect_nested(&case.body, ranges);
                }
            }
            _ => {}
        }
    }
}

fn collect_if(node: &ast::StmtIf, ranges: &mut Vec<(TextSize, TextSize)>) {
    collect_nested(&node.body, ranges);
    collect_nested(&node.orelse, ranges);
}

fn collect_try(
    body: &[ast::Stmt],
    handlers: &[ast::ExceptHandler],
    orelse: &[ast::Stmt],
    finalbody: &[ast::Stmt],
    ranges: &mut Vec<(TextSize, TextSize)>,
) {
    collect_nested(body, ranges);
    collect_nested(orelse, ranges);
    collect_nested(finalbody, ranges);
    for handler in handlers {
        let ast::ExceptHandler::ExceptHandler(handler) = handler;
        collect_nested(&handler.body, ranges);
    }
}

fn line_in_nested(line: usize, locator: &Locator, nested: &[(TextSize, TextSize)]) -> bool {
    let start = TextSize::try_from(locator.line_start_offset(line)).expect("line offset");
    nested.iter().any(|&(lo, hi)| start >= lo && start < hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Echo007Settings;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;
    use std::collections::HashSet;
    use std::path::Path;

    fn settings(exclude_tests: bool) -> Settings {
        Settings {
            enabled: HashSet::from([RULE_EMPTY_LINES.to_string()]),
            echo007: Echo007Settings { exclude_tests },
            ..Settings::default()
        }
    }

    fn lint(source: &str) -> Vec<Diagnostic> {
        lint_with(source, &settings(true))
    }

    fn lint_with(source: &str, settings: &Settings) -> Vec<Diagnostic> {
        let module = Suite::parse(source, "<test>").expect("parse");
        let locator = Locator::new(source);
        let noqa = NoqaIndex::from_source(source);
        let mut diagnostics = Vec::new();
        for stmt in module {
            lint_stmt(&locator, &noqa, settings, &stmt, &mut diagnostics);
        }
        diagnostics
    }

    fn lint_stmt(
        locator: &Locator,
        noqa: &NoqaIndex,
        settings: &Settings,
        stmt: &ast::Stmt,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        match stmt {
            ast::Stmt::FunctionDef(node) => {
                check_function_def(
                    locator,
                    noqa,
                    Path::new("t.py"),
                    settings,
                    node,
                    diagnostics,
                );
                for item in &node.body {
                    lint_stmt(locator, noqa, settings, item, diagnostics);
                }
            }
            ast::Stmt::AsyncFunctionDef(node) => {
                check_async_function_def(
                    locator,
                    noqa,
                    Path::new("t.py"),
                    settings,
                    node,
                    diagnostics,
                );
                for item in &node.body {
                    lint_stmt(locator, noqa, settings, item, diagnostics);
                }
            }
            ast::Stmt::ClassDef(node) => {
                for item in &node.body {
                    lint_stmt(locator, noqa, settings, item, diagnostics);
                }
            }
            _ => {}
        }
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
        assert_eq!(1, lint("def f():\n\n    return 1\n").len());
    }

    #[test]
    fn blank_between_methods_is_clean() {
        let source =
            "class C:\n    def f(self):\n        return 1\n\n    def g(self):\n        return 2\n";
        assert!(lint(source).is_empty());
    }

    #[test]
    fn nested_blank_is_reported_once() {
        let source = "def f():\n    def g():\n        a = 1\n\n        return a\n    return g\n";
        assert_eq!(1, lint(source).len());
    }

    #[test]
    fn test_prefixed_function_is_skipped() {
        assert!(lint("def test_f():\n    a = 1\n\n    return a\n").is_empty());
    }

    #[test]
    fn test_prefixed_can_be_enabled() {
        assert_eq!(
            1,
            lint_with(
                "def test_f():\n    a = 1\n\n    return a\n",
                &settings(false)
            )
            .len()
        );
    }
}
