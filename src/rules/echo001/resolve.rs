use rustpython_parser::ast;

use super::signature::{Signature, Slot};
use crate::bindings::Bindings;

pub(super) fn resolve(bindings: &Bindings, expr: &ast::ExprCall) -> Option<Signature> {
    if let Some(signature) = builtin_signature(bindings, expr) {
        return Some(signature);
    }
    if let Some(signature) = method_signature(expr) {
        return Some(signature);
    }
    bindings
        .function_shape(expr)
        .map(Signature::from_function_shape)
}

fn builtin_signature(bindings: &Bindings, expr: &ast::ExprCall) -> Option<Signature> {
    for name in BUILTIN_NAMES {
        if bindings.match_builtin_expr(expr, name) {
            return Some(signature_for_builtin(name));
        }
    }
    None
}

fn method_signature(expr: &ast::ExprCall) -> Option<Signature> {
    let ast::Expr::Attribute(attribute) = expr.func.as_ref() else {
        return None;
    };
    match attribute.attr.as_str() {
        "get" | "pop" | "setdefault" => Some(Signature::positional_only(2)),
        "aggregate" | "alias" | "annotate" | "dates" | "datetimes" | "defer" | "difference"
        | "distinct" | "exclude" | "filter" | "intersection" | "only" | "order_by"
        | "prefetch_related" | "select_for_update" | "select_related" | "union" | "update"
        | "values" | "values_list" => Some(Signature::varargs()),
        _ => None,
    }
}

fn signature_for_builtin(name: &str) -> Signature {
    match name {
        "enumerate" | "open" | "round" => Signature::keyword_capable(2),
        "int" | "sum" => Signature::from_slots(&[Slot::PosOnly, Slot::Normal]),
        "print" | "max" | "min" | "zip" | "map" | "filter" => Signature::varargs(),
        _ => Signature::positional_only(2),
    }
}

const BUILTIN_NAMES: &[&str] = &[
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
