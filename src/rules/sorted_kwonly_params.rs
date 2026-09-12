use rustpython_parser::ast::{self, Ranged};

use crate::RULE_SORTED_KWONLY_PARAMS;
use crate::common::report::report;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub(crate) fn check_function_def(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    stmt: &ast::StmtFunctionDef,
    diagnostics: &mut Vec<Diagnostic>,
) {
    check_args(locator, noqa, path, stmt, &stmt.args, diagnostics);
}

pub(crate) fn check_async_function_def(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    stmt: &ast::StmtAsyncFunctionDef,
    diagnostics: &mut Vec<Diagnostic>,
) {
    check_args(locator, noqa, path, stmt, &stmt.args, diagnostics);
}

fn check_args(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    node: &impl Ranged,
    arguments: &ast::Arguments,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let names = kwonly_names(arguments);
    if names.len() < 2 || is_sorted(&names) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        node,
        RULE_SORTED_KWONLY_PARAMS,
        "keyword-only parameters are not sorted",
        diagnostics,
    );
}

fn kwonly_names(arguments: &ast::Arguments) -> Vec<&str> {
    arguments
        .kwonlyargs
        .iter()
        .map(|arg| arg.def.arg.as_str())
        .collect()
}

fn is_sorted(names: &[&str]) -> bool {
    names.windows(2).all(|pair| pair[0] <= pair[1])
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
            match stmt {
                ast::Stmt::FunctionDef(node) => {
                    check_function_def(&locator, &noqa, Path::new("t.py"), &node, &mut diagnostics);
                }
                ast::Stmt::AsyncFunctionDef(node) => {
                    check_async_function_def(
                        &locator,
                        &noqa,
                        Path::new("t.py"),
                        &node,
                        &mut diagnostics,
                    );
                }
                _ => {}
            }
        }
        diagnostics
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
    fn one_kwonly_is_clean() {
        assert!(lint("def f(*, a):\n    pass\n").is_empty());
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
    fn vararg_kwarg_do_not_affect_kwonly_order() {
        assert!(lint("def f(*args, a, b, **kwargs):\n    pass\n").is_empty());
    }

    #[test]
    fn async_unsorted_kwonly_are_reported() {
        assert_eq!(
            RULE_SORTED_KWONLY_PARAMS,
            lint("async def f(*, b, a):\n    pass\n")[0].code
        );
    }
}
