use rustpython_parser::ast::{self, Ranged};

use crate::RULE_PARAMS_ONE_PER_LINE;
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
    let params = collect_params(arguments);
    let countable = params
        .iter()
        .copied()
        .filter(|param| !is_self_or_cls(param))
        .count();
    if countable <= 1 {
        return;
    }
    if params_on_separate_lines(locator, &params) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        node,
        RULE_PARAMS_ONE_PER_LINE,
        "put each parameter on its own line when a function has more than one parameter",
        diagnostics,
    );
}

fn collect_params(arguments: &ast::Arguments) -> Vec<&ast::Arg> {
    let mut params = Vec::new();
    for arg in &arguments.posonlyargs {
        params.push(&arg.def);
    }
    for arg in &arguments.args {
        params.push(&arg.def);
    }
    if let Some(vararg) = &arguments.vararg {
        params.push(vararg);
    }
    for arg in &arguments.kwonlyargs {
        params.push(&arg.def);
    }
    if let Some(kwarg) = &arguments.kwarg {
        params.push(kwarg);
    }
    params
}

fn is_self_or_cls(arg: &ast::Arg) -> bool {
    matches!(arg.arg.as_str(), "self" | "cls")
}

fn params_on_separate_lines(locator: &Locator, params: &[&ast::Arg]) -> bool {
    let mut lines = Vec::with_capacity(params.len());
    for param in params {
        let line = locator.line_index(param.start());
        if lines.contains(&line) {
            return false;
        }
        lines.push(line);
    }
    true
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
    fn two_params_separate_lines_are_clean() {
        assert!(lint("def f(\n    a,\n    b,\n):\n    pass\n").is_empty());
    }

    #[test]
    fn self_with_one_param_same_line_is_clean() {
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
    fn self_with_two_params_separate_lines_are_clean() {
        assert!(lint("def f(\n    self,\n    a,\n    b,\n):\n    pass\n").is_empty());
    }

    #[test]
    fn two_kwonly_same_line_are_reported() {
        assert_eq!(
            RULE_PARAMS_ONE_PER_LINE,
            lint("def f(*, a, b):\n    pass\n")[0].code
        );
    }

    #[test]
    fn async_two_params_same_line_are_reported() {
        assert_eq!(
            RULE_PARAMS_ONE_PER_LINE,
            lint("async def f(a, b):\n    pass\n")[0].code
        );
    }
}
