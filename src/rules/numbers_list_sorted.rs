use rustpython_parser::ast;

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::RULE_NUMBERS_LIST_SORTED;

use crate::common::report::report;
use crate::common::sort_key::{cmp_numbers, is_nondecreasing, sort_key, SortKey};

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

    if !keys.iter().all(SortKey::is_number) {
        return;
    }

    if is_nondecreasing(&keys, cmp_numbers) {
        return;
    }

    report(
        locator,
        noqa,
        path,
        expr,
        RULE_NUMBERS_LIST_SORTED,
        "numbers list is not sorted",
        diagnostics,
    );
}
