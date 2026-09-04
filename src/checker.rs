use rustpython_parser::ast::{self, Visitor};

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::rules::{mixed_list_sorted, numbers_list_sorted, words_list_sorted};

pub struct Checker<'a> {
    pub locator: &'a Locator,
    pub noqa: &'a NoqaIndex,
    pub path: &'a std::path::Path,
    pub diagnostics: &'a mut Vec<Diagnostic>,
}

impl Visitor for Checker<'_> {
    fn visit_expr_list(&mut self, node: ast::ExprList) {
        numbers_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        words_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        mixed_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        self.generic_visit_expr_list(node);
    }
}
