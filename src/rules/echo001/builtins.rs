use rustpython_parser::ast;

use crate::bindings::Bindings;

const POSITIONAL_BUILTINS: &[&str] = &[
    "abs",
    "all",
    "any",
    "bool",
    "dict",
    "filter",
    "float",
    "getattr",
    "hasattr",
    "isinstance",
    "issubclass",
    "len",
    "list",
    "map",
    "max",
    "min",
    "print",
    "range",
    "reversed",
    "set",
    "setattr",
    "sorted",
    "str",
    "tuple",
    "zip",
];

pub(super) fn is_positional_builtin(bindings: &Bindings, expr: &ast::ExprCall) -> bool {
    POSITIONAL_BUILTINS
        .iter()
        .any(|name| bindings.match_builtin_expr(expr, name))
}
