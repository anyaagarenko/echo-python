use rustpython_parser::ast;

use super::config;
use crate::common::quote;
use crate::locator::Locator;

pub(super) fn for_call(locator: &Locator, func: &ast::Expr) -> String {
    let name = config::callee_name(func).unwrap_or_else(|| locator.text(func).to_string());
    format!("use keyword arguments for {}", quote::tick(&name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;

    fn func_from(source: &str) -> ast::Expr {
        let module = Suite::parse(source, "<test>").expect("parse");
        match module.into_iter().next().expect("stmt") {
            ast::Stmt::Expr(stmt) => match *stmt.value {
                ast::Expr::Call(call) => *call.func,
                other => panic!("expected call, got {other:?}"),
            },
            other => panic!("expected expr stmt, got {other:?}"),
        }
    }

    #[test]
    fn names_bare_callee() {
        let source = "open(\"a\", \"r\")\n";
        assert_eq!(
            "use keyword arguments for `open`",
            for_call(&Locator::new(source), &func_from(source))
        );
    }

    #[test]
    fn names_method() {
        let source = "name.replace(\"a\", \"b\")\n";
        assert_eq!(
            "use keyword arguments for `replace`",
            for_call(&Locator::new(source), &func_from(source))
        );
    }
}
