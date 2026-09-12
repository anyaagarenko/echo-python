use rustpython_parser::ast;

use crate::RULE_ECHO001;
use crate::bindings::Bindings;
use crate::common::report::report;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

const DEFAULT_BUILTINS: &[&str] = &[
    "abs",
    "all",
    "any",
    "bool",
    "dict",
    "enumerate",
    "filter",
    "float",
    "getattr",
    "hasattr",
    "int",
    "isinstance",
    "issubclass",
    "len",
    "list",
    "map",
    "max",
    "min",
    "open",
    "print",
    "range",
    "reversed",
    "round",
    "set",
    "setattr",
    "sorted",
    "str",
    "sum",
    "tuple",
    "zip",
];

pub(crate) fn check(
    bindings: &Bindings,
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    expr: &ast::ExprCall,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if is_ignored_builtin(bindings, expr) {
        return;
    }
    if is_ignored_config(settings, &expr.func) {
        return;
    }
    if is_mapping_method(expr) {
        return;
    }
    if countable_positionals(&expr.args) <= 1 {
        return;
    }
    report(
        locator,
        noqa,
        path,
        expr,
        RULE_ECHO001,
        "use keyword arguments for calls with multiple positional args",
        diagnostics,
    );
}

fn is_mapping_method(expr: &ast::ExprCall) -> bool {
    let ast::Expr::Attribute(attribute) = expr.func.as_ref() else {
        return false;
    };
    matches!(attribute.attr.as_str(), "get" | "pop" | "setdefault")
        && countable_positionals(&expr.args) == 2
}

fn is_ignored_builtin(bindings: &Bindings, expr: &ast::ExprCall) -> bool {
    DEFAULT_BUILTINS
        .iter()
        .any(|name| bindings.match_builtin_expr(expr, name))
}

fn is_ignored_config(settings: &Settings, func: &ast::Expr) -> bool {
    let Some(path) = callee_path(func) else {
        return false;
    };
    let short = path.rsplit('.').next().unwrap_or(path.as_str());
    settings.echo001.ignores(path.as_str()) || settings.echo001.ignores(short)
}

fn callee_path(func: &ast::Expr) -> Option<String> {
    match func {
        ast::Expr::Name(name) => Some(name.id.to_string()),
        ast::Expr::Attribute(attr) => Some(callee_path(attr.value.as_ref()).map_or_else(
            || attr.attr.to_string(),
            |base| format!("{base}.{}", attr.attr.as_str()),
        )),
        _ => None,
    }
}

fn countable_positionals(args: &[ast::Expr]) -> usize {
    let args = skip_self_or_cls(args);
    args.iter().filter(|arg| !is_starred(arg)).count()
}

fn skip_self_or_cls(args: &[ast::Expr]) -> &[ast::Expr] {
    match args.first() {
        Some(arg) if is_self_or_cls(arg) => &args[1..],
        _ => args,
    }
}

fn is_self_or_cls(expr: &ast::Expr) -> bool {
    matches!(
        expr,
        ast::Expr::Name(name) if name.id.as_str() == "self" || name.id.as_str() == "cls"
    )
}

const fn is_starred(expr: &ast::Expr) -> bool {
    matches!(expr, ast::Expr::Starred(_))
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
    fn counts_two_positionals() {
        let call = call_from("f(1, 2)\n");

        let count = countable_positionals(&call.args);

        assert_eq!(2, count);
    }

    #[test]
    fn counts_one_positional() {
        let call = call_from("f(1)\n");

        let count = countable_positionals(&call.args);

        assert_eq!(1, count);
    }

    #[test]
    fn skips_self_then_one_arg() {
        let call = call_from("Foo.m(self, x)\n");

        let count = countable_positionals(&call.args);

        assert_eq!(1, count);
    }

    #[test]
    fn skips_self_then_two_args() {
        let call = call_from("Foo.m(self, x, y)\n");

        let count = countable_positionals(&call.args);

        assert_eq!(2, count);
    }

    #[test]
    fn ignores_starred() {
        let call = call_from("f(1, *xs)\n");

        let count = countable_positionals(&call.args);

        assert_eq!(1, count);
    }

    #[test]
    fn callee_path_from_attribute() {
        let call = call_from("obj.append(1, 2)\n");

        let path = callee_path(&call.func);

        assert_eq!(Some("obj.append".to_string()), path);
    }

    #[test]
    fn callee_path_from_nested_attribute() {
        let call = call_from("pytest.param(1, 2)\n");

        let path = callee_path(&call.func);

        assert_eq!(Some("pytest.param".to_string()), path);
    }

    #[test]
    fn callee_path_falls_back_on_call_chain() {
        let call = call_from("qs.filter(x=1).values_list(\"a\", \"b\")\n");

        let path = callee_path(&call.func);

        assert_eq!(Some("values_list".to_string()), path);
    }

    #[test]
    fn short_ignore_still_matches_call_chain() {
        let source = "qs.filter(x=1).values_list(\"a\", \"b\")\n";
        let settings = enabled_settings(&["values_list"]);

        let diagnostics = diagnose_call(source, &settings);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn qualified_ignore_skips_pytest_param() {
        let settings = enabled_settings(&["pytest.param"]);

        let diagnostics = diagnose_call("pytest.param(1, 2)\n", &settings);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn qualified_ignore_keeps_other_param() {
        let settings = enabled_settings(&["pytest.param"]);

        let diagnostics = diagnose_call("other.param(1, 2)\n", &settings);

        assert_eq!(1, diagnostics.len());
    }

    #[test]
    fn qualified_ignore_keeps_bare_param() {
        let settings = enabled_settings(&["pytest.param"]);

        let diagnostics = diagnose_call("param(1, 2)\n", &settings);

        assert_eq!(1, diagnostics.len());
    }

    #[test]
    fn isinstance_is_ignored_by_default() {
        let settings = enabled_settings(&[]);

        let diagnostics = diagnose_call("isinstance(1, int)\n", &settings);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn builtins_isinstance_is_ignored() {
        let settings = enabled_settings(&[]);

        let diagnostics = diagnose_call("builtins.isinstance(1, int)\n", &settings);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn shadowed_isinstance_is_reported() {
        let source = "isinstance = check\nisinstance(1, int)\n";
        let settings = enabled_settings(&[]);

        let diagnostics = diagnose(source, &call_at(source, 1), &settings);

        assert_eq!(1, diagnostics.len());
    }

    #[test]
    fn ignore_skips_check() {
        let settings = enabled_settings(&["print"]);

        let diagnostics = diagnose_call("print(1, 2)\n", &settings);

        assert!(diagnostics.is_empty());
    }
}
