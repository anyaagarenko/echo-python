mod cmp;
mod from_expr;
mod order;
mod predicates;

pub(crate) use cmp::{cmp_numbers, cmp_words};
pub(crate) use from_expr::sort_key;
pub(crate) use order::{is_nondecreasing, keys_match_mixed_order};

#[derive(Clone, Debug)]
pub(crate) enum SortKey {
    Float(f64),
    Int(rustpython_parser::ast::bigint::BigInt),
    Word(String),
}
