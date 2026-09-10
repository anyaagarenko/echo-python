use rustpython_parser::ast;

use crate::RULE_WORDS_LIST_SORTED;
use crate::common::report::report;
use crate::common::sort_key::{SortKey, cmp_words, is_nondecreasing, sort_key};
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
    let Some(keys) = word_keys(expr) else {
        return;
    };
    if is_nondecreasing(&keys, cmp_words) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        expr,
        RULE_WORDS_LIST_SORTED,
        "words list is not sorted",
        diagnostics,
    );
}

fn word_keys(expr: &ast::ExprList) -> Option<Vec<SortKey>> {
    if expr.elts.len() < 2 {
        return None;
    }
    let keys = expr.elts.iter().map(sort_key).collect::<Option<Vec<_>>>()?;
    keys.iter().all(SortKey::is_word).then_some(keys)
}
