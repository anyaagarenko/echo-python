use rustpython_parser::ast;

use crate::RULE_WORDS_LIST_SORTED;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

use crate::common::report::report;
use crate::common::sort_key::{SortKey, cmp_words, is_nondecreasing, sort_key};

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

    if !keys.iter().all(SortKey::is_word) {
        return;
    }

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
