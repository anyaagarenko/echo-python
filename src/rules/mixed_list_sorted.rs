use rustpython_parser::ast;

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::RULE_MIXED_LIST_SORTED;

use crate::common::report::report;
use crate::common::sort_key::{cmp_numbers, cmp_words, keys_match_mixed_order, sort_key, SortKey};

pub fn check(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    expr: &ast::ExprList,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if expr.elts.len() < 2 {
        return;
    }

    let Some(keys) = expr.elts.iter().map(sort_key).collect::<Option<Vec<_>>>() else {
        return;
    };

    let has_number = keys.iter().any(SortKey::is_number);
    let has_word = keys.iter().any(SortKey::is_word);
    if !(has_number && has_word) {
        return;
    }
    if !keys.iter().all(|key| key.is_number() || key.is_word()) {
        return;
    }

    let numbers: Vec<_> = keys.iter().filter(|key| key.is_number()).cloned().collect();
    let words: Vec<_> = keys.iter().filter(|key| key.is_word()).cloned().collect();

    let mut expected = numbers;
    expected.sort_by(cmp_numbers);
    let mut sorted_words = words;
    sorted_words.sort_by(cmp_words);
    expected.extend(sorted_words);

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
