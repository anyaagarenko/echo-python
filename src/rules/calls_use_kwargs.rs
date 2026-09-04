use rustpython_parser::ast;

use crate::RULE_CALLS_USE_KWARGS;
use crate::common::report::report;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

const DEFAULT_IGNORE: &[&str] = &[
    "getattr",
    "hasattr",
    "isinstance",
    "issubclass",
    "max",
    "min",
    "path",
    "setattr",
    "zip",
];

pub(crate) fn check(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    expr: &ast::ExprCall,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if is_ignored(settings, &expr.func) {
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
        RULE_CALLS_USE_KWARGS,
        "use keyword arguments for calls with multiple positional args",
        diagnostics,
    );
}

fn is_ignored(settings: &Settings, func: &ast::Expr) -> bool {
    callee_name(func).is_some_and(|name| {
        DEFAULT_IGNORE.contains(&name) || settings.calls_use_kwargs.ignores(name)
    })
}

fn callee_name(func: &ast::Expr) -> Option<&str> {
    match func {
        ast::Expr::Name(name) => Some(name.id.as_str()),
        ast::Expr::Attribute(attr) => Some(attr.attr.as_str()),
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
    use crate::settings::CallsUseKwargsSettings;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;
    use std::collections::HashSet;
    use std::path::Path;

    fn call_from(source: &str) -> ast::ExprCall {
        let module = Suite::parse(source, "<test>").expect("parse");
        match module.into_iter().next().expect("stmt") {
            ast::Stmt::Expr(stmt) => match *stmt.value {
                ast::Expr::Call(call) => call,
                other => panic!("expected call, got {other:?}"),
            },
            other => panic!("expected expr stmt, got {other:?}"),
        }
    }

    #[test]
    fn counts_two_positionals() {
        let call = call_from("f(1, 2)\n");
        assert_eq!(2, countable_positionals(&call.args));
    }

    #[test]
    fn counts_one_positional() {
        let call = call_from("f(1)\n");
        assert_eq!(1, countable_positionals(&call.args));
    }

    #[test]
    fn skips_self_then_one_arg() {
        let call = call_from("Foo.m(self, x)\n");
        assert_eq!(1, countable_positionals(&call.args));
    }

    #[test]
    fn skips_self_then_two_args() {
        let call = call_from("Foo.m(self, x, y)\n");
        assert_eq!(2, countable_positionals(&call.args));
    }

    #[test]
    fn ignores_starred() {
        let call = call_from("f(1, *xs)\n");
        assert_eq!(1, countable_positionals(&call.args));
    }

    #[test]
    fn callee_name_from_attribute() {
        let call = call_from("obj.append(1, 2)\n");
        assert_eq!(Some("append"), callee_name(&call.func));
    }

    #[test]
    fn isinstance_is_ignored_by_default() {
        let call = call_from("isinstance(1, int)\n");
        let settings = Settings {
            enabled: HashSet::from([RULE_CALLS_USE_KWARGS.to_string()]),
            calls_use_kwargs: CallsUseKwargsSettings::default(),
        };
        let locator = Locator::new("isinstance(1, int)\n");
        let noqa = NoqaIndex::from_source("isinstance(1, int)\n");
        let mut diagnostics = Vec::new();
        check(
            &locator,
            &noqa,
            Path::new("t.py"),
            &settings,
            &call,
            &mut diagnostics,
        );
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignore_skips_check() {
        let call = call_from("print(1, 2)\n");
        let settings = Settings {
            enabled: HashSet::from([RULE_CALLS_USE_KWARGS.to_string()]),
            calls_use_kwargs: crate::settings::CallsUseKwargsSettings {
                ignore: HashSet::from(["print".to_string()]),
            },
        };
        let locator = Locator::new("print(1, 2)\n");
        let noqa = NoqaIndex::from_source("print(1, 2)\n");
        let mut diagnostics = Vec::new();
        check(
            &locator,
            &noqa,
            Path::new("t.py"),
            &settings,
            &call,
            &mut diagnostics,
        );
        assert!(diagnostics.is_empty());
    }
}
