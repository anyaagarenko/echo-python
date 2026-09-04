use rustpython_parser::ast::{self, Ranged};

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub fn report(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    expr: &ast::ExprList,
    code: &'static str,
    message: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let row = locator.line_index(expr.start());
    if noqa.suppresses(row, code) {
        return;
    }
    let column = locator.column_index(expr.start());
    diagnostics.push(Diagnostic::new(path, row, column, code, message));
}
