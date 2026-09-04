use std::cmp::Ordering;

use rustpython_parser::ast;

use super::SortKey;

pub(crate) fn cmp_numbers(left: &SortKey, right: &SortKey) -> Ordering {
    match (left, right) {
        (SortKey::Int(a), SortKey::Int(b)) => a.cmp(b),
        (SortKey::Float(a), SortKey::Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
        (SortKey::Int(a), SortKey::Float(b)) => cmp_int_float(a, *b),
        (SortKey::Float(a), SortKey::Int(b)) => cmp_int_float(b, *a).reverse(),
        _ => Ordering::Equal,
    }
}

pub(crate) fn cmp_words(left: &SortKey, right: &SortKey) -> Ordering {
    match (left, right) {
        (SortKey::Word(a), SortKey::Word(b)) => a.cmp(b),
        _ => Ordering::Equal,
    }
}

fn int_to_f64(value: &ast::bigint::BigInt) -> Option<f64> {
    let parsed: f64 = value.to_string().parse().ok()?;
    parsed.is_finite().then_some(parsed)
}

pub(super) fn cmp_int_float(int: &ast::bigint::BigInt, float: f64) -> Ordering {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::ast::bigint::BigInt;

    #[test]
    fn compares_ints() {
        assert_eq!(
            Ordering::Less,
            cmp_numbers(
                &SortKey::Int(BigInt::from(1)),
                &SortKey::Int(BigInt::from(2))
            )
        );
    }

    #[test]
    fn compares_words() {
        assert_eq!(
            Ordering::Less,
            cmp_words(&SortKey::Word("a".into()), &SortKey::Word("b".into()))
        );
    }
}
