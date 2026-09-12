use rustpython_parser::ast::{self, Visitor};

use super::Checker;
use crate::RULE_ECHO001;
use crate::RULE_MIXED_LIST_SORTED;
use crate::RULE_NUMBERS_LIST_SORTED;
use crate::RULE_PARAMS_ONE_PER_LINE;
use crate::RULE_WORDS_LIST_SORTED;
use crate::rules::{
    echo001, mixed_list_sorted, numbers_list_sorted, params_one_per_line, words_list_sorted,
};

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
        if self.settings.is_enabled(RULE_ECHO001) {
            echo001::check(
                self.bindings,
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

    fn visit_stmt_function_def(&mut self, node: ast::StmtFunctionDef) {
        if self.settings.is_enabled(RULE_PARAMS_ONE_PER_LINE) {
            params_one_per_line::check_function_def(
                self.locator,
                self.noqa,
                self.path,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_stmt_function_def(node);
    }

    fn visit_stmt_async_function_def(&mut self, node: ast::StmtAsyncFunctionDef) {
        if self.settings.is_enabled(RULE_PARAMS_ONE_PER_LINE) {
            params_one_per_line::check_async_function_def(
                self.locator,
                self.noqa,
                self.path,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_stmt_async_function_def(node);
    }
}
