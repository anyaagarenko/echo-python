use std::cmp::Ordering;

use rustpython_parser::ast::{self, Constant, UnaryOp};

#[derive(Clone, Debug)]
pub enum SortKey {
    Float(f64),
    Int(ast::bigint::BigInt),
    Word(String),
}

impl SortKey {
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    pub const fn is_word(&self) -> bool {
        matches!(self, Self::Word(_))
    }
}

pub fn sort_key(expr: &ast::Expr) -> Option<SortKey> {
    match expr {
        ast::Expr::Constant(c) => match &c.value {
            Constant::Int(value) => Some(SortKey::Int(value.clone())),
            Constant::Float(value) if value.is_finite() => Some(SortKey::Float(*value)),
            Constant::Bool(value) => {
                Some(SortKey::Int(ast::bigint::BigInt::from(usize::from(*value))))
            }
            Constant::Str(value) => Some(SortKey::Word(value.clone())),
            _ => None,
        },
        ast::Expr::Name(name) => Some(SortKey::Word(name.id.to_string())),
        ast::Expr::UnaryOp(unary) => match unary.op {
            UnaryOp::UAdd => sort_key(unary.operand.as_ref()),
            UnaryOp::USub => match sort_key(unary.operand.as_ref())? {
                SortKey::Int(value) => Some(SortKey::Int(-value)),
                SortKey::Float(value) => Some(SortKey::Float(-value)),
                SortKey::Word(_) => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn int_to_f64(value: &ast::bigint::BigInt) -> Option<f64> {
    let parsed: f64 = value.to_string().parse().ok()?;
    parsed.is_finite().then_some(parsed)
}

fn cmp_int_float(int: &ast::bigint::BigInt, float: f64) -> Ordering {
    if !float.is_finite() {
        return Ordering::Equal;
    }
    if let Some(as_float) = int_to_f64(int) {
        return as_float.partial_cmp(&float).unwrap_or(Ordering::Equal);
    }
    match (*int).cmp(&ast::bigint::BigInt::from(0)) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => 0.0_f64.partial_cmp(&float).unwrap_or(Ordering::Equal),
    }
}

pub fn cmp_numbers(left: &SortKey, right: &SortKey) -> Ordering {
    match (left, right) {
        (SortKey::Int(a), SortKey::Int(b)) => a.cmp(b),
        (SortKey::Float(a), SortKey::Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
        (SortKey::Int(a), SortKey::Float(b)) => cmp_int_float(a, *b),
        (SortKey::Float(a), SortKey::Int(b)) => cmp_int_float(b, *a).reverse(),
        _ => Ordering::Equal,
    }
}

pub fn cmp_words(left: &SortKey, right: &SortKey) -> Ordering {
    match (left, right) {
        (SortKey::Word(a), SortKey::Word(b)) => a.cmp(b),
        _ => Ordering::Equal,
    }
}

pub fn is_nondecreasing(keys: &[SortKey], cmp: fn(&SortKey, &SortKey) -> Ordering) -> bool {
    keys.windows(2)
        .all(|pair| cmp(&pair[0], &pair[1]) != Ordering::Greater)
}

pub fn keys_match_mixed_order(actual: &[SortKey], expected: &[SortKey]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    actual
        .iter()
        .zip(expected.iter())
        .all(|(a, e)| match (a, e) {
            (SortKey::Int(x), SortKey::Int(y)) => x == y,
            (SortKey::Float(x), SortKey::Float(y)) => x.to_bits() == y.to_bits(),
            (SortKey::Int(x), SortKey::Float(y)) | (SortKey::Float(y), SortKey::Int(x)) => {
                cmp_int_float(x, *y) == Ordering::Equal
            }
            (SortKey::Word(x), SortKey::Word(y)) => x == y,
            _ => false,
        })
}
