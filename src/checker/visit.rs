use rustpython_parser::ast::{self, Visitor};

use super::Checker;
use crate::rules::{calls_use_kwargs, mixed_list_sorted, numbers_list_sorted, words_list_sorted};

impl Visitor for Checker<'_> {
    fn visit_expr_list(&mut self, node: ast::ExprList) {
        numbers_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        words_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        mixed_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        self.generic_visit_expr_list(node);
    }

    fn visit_expr_call(&mut self, node: ast::ExprCall) {
        calls_use_kwargs::check(
            self.locator,
            self.noqa,
            self.path,
            self.settings,
            &node,
            self.diagnostics,
        );
        self.generic_visit_expr_call(node);
    }
}
