use rustpython_parser::ast::Ranged;

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub(crate) fn report(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    node: &impl Ranged,
    code: &'static str,
    message: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let row = locator.line_index(node.start());
    let column = locator.column_index(node.start());
    report_at(noqa, path, row, column, code, message, diagnostics);
}

pub(crate) fn report_at(
    noqa: &NoqaIndex,
    path: &std::path::Path,
    row: usize,
    column: usize,
    code: &'static str,
    message: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if noqa.suppresses(row, code) {
        return;
    }
    diagnostics.push(Diagnostic::new(path, row, column, code, message));
}
