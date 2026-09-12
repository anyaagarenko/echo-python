use rustpython_parser::ast;

use super::positionals;

pub(super) fn is_positional_mapping_method(expr: &ast::ExprCall) -> bool {
    let ast::Expr::Attribute(attribute) = expr.func.as_ref() else {
        return false;
    };
    matches!(attribute.attr.as_str(), "get" | "pop" | "setdefault")
        && positionals::countable(&expr.args) == 2
}
