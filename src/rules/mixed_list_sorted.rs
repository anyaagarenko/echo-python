use rustpython_parser::ast;

use crate::RULE_MIXED_LIST_SORTED;
use crate::common::report::report;
use crate::common::sort_key::{SortKey, cmp_numbers, cmp_words, keys_match_mixed_order, sort_key};
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub(crate) fn check(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    expr: &ast::ExprList,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(keys) = mixed_keys(expr) else {
        return;
    };
    let expected = expected_mixed_order(&keys);
    if keys_match_mixed_order(&keys, &expected) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        expr,
        RULE_MIXED_LIST_SORTED,
        "mixed list is not sorted (numbers then words)",
        diagnostics,
    );
}

fn mixed_keys(expr: &ast::ExprList) -> Option<Vec<SortKey>> {
    if expr.elts.len() < 2 {
        return None;
    }
    let keys = expr.elts.iter().map(sort_key).collect::<Option<Vec<_>>>()?;
    let has_number = keys.iter().any(SortKey::is_number);
    let has_word = keys.iter().any(SortKey::is_word);
    if !(has_number && has_word) {
        return None;
    }
    if !keys.iter().all(|key| key.is_number() || key.is_word()) {
        return None;
    }
    Some(keys)
}

fn expected_mixed_order(keys: &[SortKey]) -> Vec<SortKey> {
    let mut numbers: Vec<_> = keys.iter().filter(|key| key.is_number()).cloned().collect();
    let mut words: Vec<_> = keys.iter().filter(|key| key.is_word()).cloned().collect();
    numbers.sort_by(cmp_numbers);
    words.sort_by(cmp_words);
    numbers.extend(words);
    numbers
}
