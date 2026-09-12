use rustpython_parser::ast::{self, Visitor};

use super::Checker;
use crate::RULE_ECHO001;
use crate::RULE_PARAMS_ONE_PER_LINE;
use crate::rules::{echo001, params_one_per_line, sorted_literals};

impl Visitor for Checker<'_> {
    fn visit_expr_list(&mut self, node: ast::ExprList) {
        sorted_literals::check(
            self.locator,
            self.noqa,
            self.path,
            self.settings,
            &node.elts,
            &node,
            self.diagnostics,
        );
        self.generic_visit_expr_list(node);
    }

    fn visit_expr_tuple(&mut self, node: ast::ExprTuple) {
        sorted_literals::check(
            self.locator,
            self.noqa,
            self.path,
            self.settings,
            &node.elts,
            &node,
            self.diagnostics,
        );
        self.generic_visit_expr_tuple(node);
    }

    fn visit_expr_set(&mut self, node: ast::ExprSet) {
        sorted_literals::check(
            self.locator,
            self.noqa,
            self.path,
            self.settings,
            &node.elts,
            &node,
            self.diagnostics,
        );
        self.generic_visit_expr_set(node);
    }

    fn visit_expr_dict(&mut self, node: ast::ExprDict) {
        if let Some(keys) = dict_keys(&node) {
            sorted_literals::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                keys,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_dict(node);
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

fn dict_keys(dict: &ast::ExprDict) -> Option<Vec<&ast::Expr>> {
    dict.keys
        .iter()
        .map(|key| key.as_ref())
        .collect::<Option<Vec<_>>>()
}
