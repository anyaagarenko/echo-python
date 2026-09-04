use rustpython_parser::ast::{self, Visitor};

use super::Checker;
use crate::RULE_CALLS_USE_KWARGS;
use crate::RULE_MIXED_LIST_SORTED;
use crate::RULE_NUMBERS_LIST_SORTED;
use crate::RULE_WORDS_LIST_SORTED;
use crate::rules::{calls_use_kwargs, mixed_list_sorted, numbers_list_sorted, words_list_sorted};

impl Visitor for Checker<'_> {
    fn visit_expr_list(&mut self, node: ast::ExprList) {
        if self.settings.is_enabled(RULE_NUMBERS_LIST_SORTED) {
            numbers_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        }
        if self.settings.is_enabled(RULE_WORDS_LIST_SORTED) {
            words_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        }
        if self.settings.is_enabled(RULE_MIXED_LIST_SORTED) {
            mixed_list_sorted::check(self.locator, self.noqa, self.path, &node, self.diagnostics);
        }
        self.generic_visit_expr_list(node);
    }

    fn visit_expr_call(&mut self, node: ast::ExprCall) {
        if self.settings.is_enabled(RULE_CALLS_USE_KWARGS) {
            calls_use_kwargs::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_call(node);
    }
}
