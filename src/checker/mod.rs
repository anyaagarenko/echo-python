mod visit;

use crate::bindings::Bindings;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;
use rustpython_parser::ast::{self, Visitor};

pub(crate) struct Checker<'a> {
    pub(crate) annotation_depth: u32,
    pub(crate) bindings: &'a Bindings,
    pub(crate) diagnostics: &'a mut Vec<Diagnostic>,
    pub(crate) locator: &'a Locator,
    pub(crate) noqa: &'a NoqaIndex,
    pub(crate) path: &'a std::path::Path,
    pub(crate) pytest_rows_depth: u32,
    pub(crate) settings: &'a Settings,
}

impl Checker<'_> {
    pub(crate) const fn skips_sorted_literals(&self) -> bool {
        self.annotation_depth > 0 || self.pytest_rows_depth > 0
    }

    fn visit_annotation(&mut self, expr: ast::Expr) {
        self.annotation_depth += 1;
        self.visit_expr(expr);
        self.annotation_depth -= 1;
    }
}
