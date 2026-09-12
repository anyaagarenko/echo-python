mod config;
mod positionals;
mod resolve;
mod signature;

use rustpython_parser::ast;

use crate::RULE_ECHO001;
use crate::bindings::Bindings;
use crate::common::report::report;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

pub(crate) fn check(
    bindings: &Bindings,
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    expr: &ast::ExprCall,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if config::is_ignored(settings, &expr.func) {
        return;
    }
    let countable = positionals::countable(&expr.args);
    if countable <= 1 {
        return;
    }
    if !kwargs_possible(bindings, expr, countable) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        expr,
        RULE_ECHO001,
        "use keyword arguments for multi-arg calls when keywords are possible",
        diagnostics,
    );
}

fn kwargs_possible(bindings: &Bindings, expr: &ast::ExprCall, countable: usize) -> bool {
    resolve::resolve(bindings, expr)
        .is_none_or(|signature| signature.binds_keyword_capable(countable))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::Bindings;
    use crate::settings::Echo001Settings;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;
    use std::collections::HashSet;
    use std::path::Path;

    fn call_from(source: &str) -> ast::ExprCall {
        call_at(source, 0)
    }

    fn call_at(source: &str, index: usize) -> ast::ExprCall {
        let module = Suite::parse(source, "<test>").expect("parse");
        match module.into_iter().nth(index).expect("stmt") {
            ast::Stmt::Expr(stmt) => match *stmt.value {
                ast::Expr::Call(call) => call,
                other => panic!("expected call, got {other:?}"),
            },
            other => panic!("expected expr stmt, got {other:?}"),
        }
    }

    fn bindings_from(source: &str) -> Bindings {
        let module = Suite::parse(source, "<test>").expect("parse");
        Bindings::from_module(&module)
    }

    fn enabled_settings(ignore: &[&str]) -> Settings {
        Settings {
            enabled: HashSet::from([RULE_ECHO001.to_string()]),
            echo001: Echo001Settings {
                ignore: ignore.iter().map(|name| (*name).to_string()).collect(),
            },
        }
    }

    fn diagnose(source: &str, call: &ast::ExprCall, settings: &Settings) -> Vec<Diagnostic> {
        let locator = Locator::new(source);
        let noqa = NoqaIndex::from_source(source);
        let mut diagnostics = Vec::new();
        check(
            &bindings_from(source),
            &locator,
            &noqa,
            Path::new("t.py"),
            settings,
            call,
            &mut diagnostics,
        );
        diagnostics
    }

    fn diagnose_call(source: &str, settings: &Settings) -> Vec<Diagnostic> {
        diagnose(source, &call_from(source), settings)
    }

    #[test]
    fn isinstance_is_skipped() {
        assert!(diagnose_call("isinstance(1, int)\n", &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn builtins_isinstance_is_skipped() {
        assert!(diagnose_call("builtins.isinstance(1, int)\n", &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn shadowed_isinstance_is_reported() {
        let source = "isinstance = check\nisinstance(1, int)\n";
        assert_eq!(
            1,
            diagnose(source, &call_at(source, 1), &enabled_settings(&[])).len()
        );
    }

    #[test]
    fn open_with_two_positionals_is_reported() {
        assert_eq!(
            1,
            diagnose_call("open(\"a\", \"r\")\n", &enabled_settings(&[])).len()
        );
    }

    #[test]
    fn print_with_two_positionals_is_skipped() {
        assert!(diagnose_call("print(1, 2)\n", &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn mapping_get_is_skipped() {
        assert!(diagnose_call("data.get(\"k\", None)\n", &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn local_posonly_def_is_skipped() {
        let source = "def f(a, b, /):\n    pass\nf(1, 2)\n";
        assert!(diagnose(source, &call_at(source, 1), &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn local_normal_def_is_reported() {
        let source = "def f(a, b):\n    pass\nf(1, 2)\n";
        assert_eq!(
            1,
            diagnose(source, &call_at(source, 1), &enabled_settings(&[])).len()
        );
    }

    #[test]
    fn local_varargs_def_is_skipped() {
        let source = "def f(*args):\n    pass\nf(1, 2)\n";
        assert!(diagnose(source, &call_at(source, 1), &enabled_settings(&[])).is_empty());
    }

    #[test]
    fn qualified_ignore_skips_pytest_param() {
        assert!(
            diagnose_call("pytest.param(1, 2)\n", &enabled_settings(&["pytest.param"])).is_empty()
        );
    }
}
