use rustpython_parser::ast::{self, Ranged};

use super::classify::{Kind, classify};
use crate::RULE_MIXED_LIST_SORTED;
use crate::RULE_NUMBERS_LIST_SORTED;
use crate::RULE_WORDS_LIST_SORTED;
use crate::common::report::report;
use crate::common::sort_key::{
    SortKey, cmp_numbers, cmp_words, is_nondecreasing, keys_match_mixed_order, sort_key,
};
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

pub(crate) fn check<'a, I>(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    elts: I,
    node: &impl Ranged,
    diagnostics: &mut Vec<Diagnostic>,
) where
    I: IntoIterator<Item = &'a ast::Expr>,
{
    let Some(keys) = keys_from_elts(elts) else {
        return;
    };
    let Some(kind) = classify(&keys) else {
        return;
    };
    let (code, message) = rule_for(kind);
    if !settings.is_enabled(code) {
        return;
    }
    if is_ordered(&keys, kind) {
        return;
    }
    report(locator, noqa, path, node, code, message, diagnostics);
}

fn keys_from_elts<'a, I>(elts: I) -> Option<Vec<SortKey>>
where
    I: IntoIterator<Item = &'a ast::Expr>,
{
    let elts: Vec<_> = elts.into_iter().collect();
    if elts.len() < 2 {
        return None;
    }
    elts.iter().copied().map(sort_key).collect()
}

const fn rule_for(kind: Kind) -> (&'static str, &'static str) {
    match kind {
        Kind::Numbers => (RULE_NUMBERS_LIST_SORTED, "numeric literals are not sorted"),
        Kind::Words => (RULE_WORDS_LIST_SORTED, "word literals are not sorted"),
        Kind::Mixed => (
            RULE_MIXED_LIST_SORTED,
            "mixed literals are not sorted (numbers then words)",
        ),
    }
}

fn is_ordered(keys: &[SortKey], kind: Kind) -> bool {
    match kind {
        Kind::Numbers => is_nondecreasing(keys, cmp_numbers),
        Kind::Words => is_nondecreasing(keys, cmp_words),
        Kind::Mixed => keys_match_mixed_order(keys, &expected_mixed_order(keys)),
    }
}

fn expected_mixed_order(keys: &[SortKey]) -> Vec<SortKey> {
    let mut numbers: Vec<_> = keys.iter().filter(|key| key.is_number()).cloned().collect();
    let mut words: Vec<_> = keys.iter().filter(|key| key.is_word()).cloned().collect();
    numbers.sort_by(cmp_numbers);
    words.sort_by(cmp_words);
    numbers.extend(words);
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::ast::bigint::BigInt;

    #[test]
    fn ordered_numbers_pass() {
        let keys = [SortKey::Int(BigInt::from(1)), SortKey::Int(BigInt::from(2))];

        assert!(is_ordered(&keys, Kind::Numbers));
    }

    #[test]
    fn unordered_numbers_fail() {
        let keys = [SortKey::Int(BigInt::from(2)), SortKey::Int(BigInt::from(1))];

        assert!(!is_ordered(&keys, Kind::Numbers));
    }

    #[test]
    fn ordered_mixed_pass() {
        let keys = [SortKey::Int(BigInt::from(1)), SortKey::Word("a".into())];

        assert!(is_ordered(&keys, Kind::Mixed));
    }
}
