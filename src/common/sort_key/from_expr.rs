use rustpython_parser::ast::{self, Constant, UnaryOp};

use super::SortKey;

pub(crate) fn sort_key(expr: &ast::Expr) -> Option<SortKey> {
    match expr {
        ast::Expr::Constant(c) => constant_key(&c.value),
        ast::Expr::Name(name) => Some(SortKey::Word(name.id.to_string())),
        ast::Expr::UnaryOp(unary) => unary_key(unary),
        _ => None,
    }
}

fn constant_key(value: &Constant) -> Option<SortKey> {
    match value {
        Constant::Int(value) => Some(SortKey::Int(value.clone())),
        Constant::Float(value) if value.is_finite() => Some(SortKey::Float(*value)),
        Constant::Bool(value) => Some(SortKey::Int(ast::bigint::BigInt::from(usize::from(*value)))),
        Constant::Str(value) => Some(SortKey::Word(value.clone())),
        _ => None,
    }
}

fn unary_key(unary: &ast::ExprUnaryOp) -> Option<SortKey> {
    match unary.op {
        UnaryOp::UAdd => sort_key(unary.operand.as_ref()),
        UnaryOp::USub => negate_key(sort_key(unary.operand.as_ref())?),
        _ => None,
    }
}

fn negate_key(key: SortKey) -> Option<SortKey> {
    match key {
        SortKey::Int(value) => Some(SortKey::Int(-value)),
        SortKey::Float(value) => Some(SortKey::Float(-value)),
        SortKey::Word(_) => None,
    }
}
