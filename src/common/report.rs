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
    if noqa.suppresses(row, code) {
        return;
    }
    let column = locator.column_index(node.start());
    diagnostics.push(Diagnostic::new(path, row, column, code, message));
}
