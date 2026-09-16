use rustpython_parser::ast::{self, Visitor};

use super::Checker;
use crate::RULE_BANNED_NAMES;
use crate::RULE_CLASS_ATTRIBUTE_EMPTY_LINES;
use crate::RULE_ECHO001;
use crate::RULE_EMPTY_LINES;
use crate::RULE_SORTED_KWONLY_PARAMS;
use crate::rules::{
    banned_names, class_attribute_empty_lines, echo001, empty_lines, sorted_kwonly_params,
    sorted_literals,
};

impl Visitor for Checker<'_> {
    fn visit_expr_list(&mut self, node: ast::ExprList) {
        if !self.skips_sorted_literals() {
            sorted_literals::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node.elts,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_list(node);
    }

    fn visit_expr_tuple(&mut self, node: ast::ExprTuple) {
        if !self.skips_sorted_literals() {
            sorted_literals::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node.elts,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_tuple(node);
    }

    fn visit_expr_set(&mut self, node: ast::ExprSet) {
        if !self.skips_sorted_literals() {
            sorted_literals::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node.elts,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_set(node);
    }

    fn visit_expr_dict(&mut self, node: ast::ExprDict) {
        if !self.skips_sorted_literals()
            && let Some(keys) = dict_keys(&node)
        {
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
        let pytest_rows = is_pytest_rows_callee(&node.func);
        if pytest_rows {
            self.pytest_rows_depth += 1;
        }
        self.generic_visit_expr_call(node);
        if pytest_rows {
            self.pytest_rows_depth -= 1;
        }
    }

    fn visit_expr_name(&mut self, node: ast::ExprName) {
        if self.settings.is_enabled(RULE_BANNED_NAMES) && node.ctx.is_store() {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                node.id.as_str(),
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_expr_name(node);
    }

    fn visit_stmt_class_def(&mut self, node: ast::StmtClassDef) {
        if self.settings.is_enabled(RULE_CLASS_ATTRIBUTE_EMPTY_LINES) {
            class_attribute_empty_lines::check_class_def(
                self.locator,
                self.noqa,
                self.path,
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_stmt_class_def(node);
    }

    fn visit_stmt_function_def(&mut self, node: ast::StmtFunctionDef) {
        if self.settings.is_enabled(RULE_SORTED_KWONLY_PARAMS) {
            sorted_kwonly_params::check_function_def(
                self.locator,
                self.noqa,
                self.path,
                &node,
                self.diagnostics,
            );
        }
        if self.settings.is_enabled(RULE_EMPTY_LINES) {
            empty_lines::check_function_def(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node,
                self.diagnostics,
            );
        }
        self.visit_function_parts(
            *node.args,
            node.body,
            node.decorator_list,
            node.returns.map(|value| *value),
            node.type_params,
        );
    }

    fn visit_stmt_async_function_def(&mut self, node: ast::StmtAsyncFunctionDef) {
        if self.settings.is_enabled(RULE_SORTED_KWONLY_PARAMS) {
            sorted_kwonly_params::check_async_function_def(
                self.locator,
                self.noqa,
                self.path,
                &node,
                self.diagnostics,
            );
        }
        if self.settings.is_enabled(RULE_EMPTY_LINES) {
            empty_lines::check_async_function_def(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                &node,
                self.diagnostics,
            );
        }
        self.visit_function_parts(
            *node.args,
            node.body,
            node.decorator_list,
            node.returns.map(|value| *value),
            node.type_params,
        );
    }

    fn visit_stmt_ann_assign(&mut self, node: ast::StmtAnnAssign) {
        self.visit_expr(*node.target);
        self.visit_annotation(*node.annotation);
        if let Some(value) = node.value {
            self.visit_expr(*value);
        }
    }

    fn visit_stmt_type_alias(&mut self, node: ast::StmtTypeAlias) {
        self.visit_expr(*node.name);
        for type_param in node.type_params {
            self.visit_type_param(type_param);
        }
        self.visit_annotation(*node.value);
    }

    fn visit_stmt_import(&mut self, node: ast::StmtImport) {
        if self.settings.is_enabled(RULE_BANNED_NAMES) {
            for alias in &node.names {
                self.check_import_alias(alias);
            }
        }
        self.generic_visit_stmt_import(node);
    }

    fn visit_stmt_import_from(&mut self, node: ast::StmtImportFrom) {
        if self.settings.is_enabled(RULE_BANNED_NAMES) {
            for alias in &node.names {
                self.check_import_alias(alias);
            }
        }
        self.generic_visit_stmt_import_from(node);
    }

    fn visit_withitem(&mut self, node: ast::WithItem) {
        self.visit_expr(node.context_expr);
        if let Some(vars) = node.optional_vars {
            self.visit_expr(*vars);
        }
    }

    fn visit_match_case(&mut self, node: ast::MatchCase) {
        self.visit_pattern(node.pattern);
        if let Some(guard) = node.guard {
            self.visit_expr(*guard);
        }
        for statement in node.body {
            self.visit_stmt(statement);
        }
    }

    fn visit_excepthandler_except_handler(&mut self, node: ast::ExceptHandlerExceptHandler) {
        if self.settings.is_enabled(RULE_BANNED_NAMES)
            && let Some(name) = &node.name
        {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                name.as_str(),
                &node,
                self.diagnostics,
            );
        }
        if let Some(type_) = node.type_ {
            self.visit_annotation(*type_);
        }
        for statement in node.body {
            self.visit_stmt(statement);
        }
    }

    fn visit_arg(&mut self, node: ast::Arg) {
        if self.settings.is_enabled(RULE_BANNED_NAMES) {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                node.arg.as_str(),
                &node,
                self.diagnostics,
            );
        }
        if let Some(annotation) = node.annotation {
            self.visit_annotation(*annotation);
        }
    }

    fn visit_arguments(&mut self, node: ast::Arguments) {
        for arg in node
            .posonlyargs
            .into_iter()
            .chain(node.args)
            .chain(node.kwonlyargs)
        {
            self.visit_arg(arg.def);
            if let Some(default) = arg.default {
                self.visit_expr(*default);
            }
        }
        if let Some(arg) = node.vararg {
            self.visit_arg(*arg);
        }
        if let Some(arg) = node.kwarg {
            self.visit_arg(*arg);
        }
    }

    fn visit_pattern_match_as(&mut self, node: ast::PatternMatchAs) {
        if self.settings.is_enabled(RULE_BANNED_NAMES)
            && let Some(name) = &node.name
        {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                name.as_str(),
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_pattern_match_as(node);
    }

    fn visit_pattern_match_star(&mut self, node: ast::PatternMatchStar) {
        if self.settings.is_enabled(RULE_BANNED_NAMES)
            && let Some(name) = &node.name
        {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                name.as_str(),
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_pattern_match_star(node);
    }

    fn visit_pattern_match_mapping(&mut self, node: ast::PatternMatchMapping) {
        if self.settings.is_enabled(RULE_BANNED_NAMES)
            && let Some(name) = &node.rest
        {
            banned_names::check(
                self.locator,
                self.noqa,
                self.path,
                self.settings,
                name.as_str(),
                &node,
                self.diagnostics,
            );
        }
        self.generic_visit_pattern_match_mapping(node);
    }
}

impl Checker<'_> {
    fn visit_function_parts(
        &mut self,
        args: ast::Arguments,
        body: Vec<ast::Stmt>,
        decorators: Vec<ast::Expr>,
        returns: Option<ast::Expr>,
        type_params: Vec<ast::TypeParam>,
    ) {
        self.visit_arguments(args);
        for statement in body {
            self.visit_stmt(statement);
        }
        for decorator in decorators {
            self.visit_expr(decorator);
        }
        if let Some(returns) = returns {
            self.visit_annotation(returns);
        }
        for type_param in type_params {
            self.visit_type_param(type_param);
        }
    }

    fn check_import_alias(&mut self, alias: &ast::Alias) {
        banned_names::check(
            self.locator,
            self.noqa,
            self.path,
            self.settings,
            import_bound_name(alias),
            alias,
            self.diagnostics,
        );
    }
}

fn import_bound_name(alias: &ast::Alias) -> &str {
    if let Some(name) = &alias.asname {
        return name.as_str();
    }
    alias.name.split('.').next().unwrap_or(alias.name.as_str())
}

fn dict_keys(dict: &ast::ExprDict) -> Option<Vec<&ast::Expr>> {
    dict.keys
        .iter()
        .map(|key| key.as_ref())
        .collect::<Option<Vec<_>>>()
}

fn is_pytest_rows_callee(func: &ast::Expr) -> bool {
    match func {
        ast::Expr::Name(name) => matches!(name.id.as_str(), "parametrize" | "param"),
        ast::Expr::Attribute(attribute) => {
            matches!(attribute.attr.as_str(), "parametrize" | "param")
        }
        _ => false,
    }
}
