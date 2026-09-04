use std::cmp::Ordering;

use super::SortKey;
use super::cmp::cmp_int_float;

pub(crate) fn is_nondecreasing(keys: &[SortKey], cmp: fn(&SortKey, &SortKey) -> Ordering) -> bool {
    keys.windows(2)
        .all(|pair| cmp(&pair[0], &pair[1]) != Ordering::Greater)
}

pub(crate) fn keys_match_mixed_order(actual: &[SortKey], expected: &[SortKey]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    actual
        .iter()
        .zip(expected.iter())
        .all(|(a, e)| keys_equal(a, e))
}

fn keys_equal(left: &SortKey, right: &SortKey) -> bool {
    match (left, right) {
        (SortKey::Int(x), SortKey::Int(y)) => x == y,
        (SortKey::Float(x), SortKey::Float(y)) => x.to_bits() == y.to_bits(),
        (SortKey::Int(x), SortKey::Float(y)) | (SortKey::Float(y), SortKey::Int(x)) => {
            cmp_int_float(x, *y) == Ordering::Equal
        }
        (SortKey::Word(x), SortKey::Word(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::sort_key::cmp_numbers;
    use rustpython_parser::ast::bigint::BigInt;

    #[test]
    fn nondecreasing_numbers() {
        let keys = [SortKey::Int(BigInt::from(1)), SortKey::Int(BigInt::from(2))];
        assert!(is_nondecreasing(&keys, cmp_numbers));
    }

    #[test]
    fn decreasing_numbers() {
        let keys = [SortKey::Int(BigInt::from(2)), SortKey::Int(BigInt::from(1))];
        assert!(!is_nondecreasing(&keys, cmp_numbers));
    }
}
