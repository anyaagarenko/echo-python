use std::collections::{HashMap, HashSet};

use rustpython_parser::ast::{self, Visitor};
use rustpython_parser::text_size::TextSize;

#[derive(Debug)]
struct Scope {
    names: HashSet<String>,
    parent: Option<usize>,
}

#[derive(Debug)]
pub(crate) struct Bindings {
    call_scopes: HashMap<TextSize, usize>,
    scopes: Vec<Scope>,
}

impl Bindings {
    pub(crate) fn from_module(module: &[ast::Stmt]) -> Self {
        let mut collector = Collector::new();
        for statement in module {
            collector.visit_stmt(statement.clone());
        }

        collector.bindings
    }

    pub(crate) fn match_builtin_expr(&self, call: &ast::ExprCall, builtin: &str) -> bool {
        let Some(scope) = self.call_scopes.get(&call.range.start()) else {
            return bare_or_builtins_attr(call.func.as_ref(), builtin);
        };
        match_builtin_expr(call.func.as_ref(), builtin, *scope, self)
    }

    fn is_unbound(&self, name: &str, mut scope: usize) -> bool {
        loop {
            if self.scopes[scope].names.contains(name) {
                return false;
            }
            scope = match self.scopes[scope].parent {
                Some(parent) => parent,
                None => return true,
            };
        }
    }
}

struct Collector {
    bindings: Bindings,
    current_scope: usize,
}

impl Collector {
    fn new() -> Self {
        Self {
            bindings: Bindings {
                call_scopes: HashMap::new(),
                scopes: vec![Scope {
                    names: HashSet::new(),
                    parent: None,
                }],
            },
            current_scope: 0,
        }
    }

    fn enter_scope(&mut self) -> usize {
        let parent = self.current_scope;
        self.bindings.scopes.push(Scope {
            names: HashSet::new(),
            parent: Some(parent),
        });
        self.current_scope = self.bindings.scopes.len() - 1;

        parent
    }

    const fn leave_scope(&mut self, parent: usize) {
        self.current_scope = parent;
    }

    fn bind(&mut self, name: &str) {
        self.bindings.scopes[self.current_scope]
            .names
            .insert(name.to_string());
    }

    fn bind_arguments(&mut self, arguments: &ast::Arguments) {
        for argument in arguments
            .posonlyargs
            .iter()
            .chain(&arguments.args)
            .chain(&arguments.kwonlyargs)
        {
            self.bind(argument.def.arg.as_str());
        }
        if let Some(argument) = &arguments.vararg {
            self.bind(argument.arg.as_str());
        }
        if let Some(argument) = &arguments.kwarg {
            self.bind(argument.arg.as_str());
        }
    }

    fn visit_function_outer(
        &mut self,
        arguments: &ast::Arguments,
        decorators: &[ast::Expr],
        returns: Option<&ast::Expr>,
        type_params: &[ast::TypeParam],
    ) {
        self.visit_arguments(arguments.clone());
        for decorator in decorators {
            self.visit_expr(decorator.clone());
        }
        if let Some(returns) = returns {
            self.visit_expr(returns.clone());
        }
        for type_param in type_params {
            self.visit_type_param(type_param.clone());
        }
    }
}

impl Visitor for Collector {
    fn visit_expr_call(&mut self, node: ast::ExprCall) {
        self.bindings
            .call_scopes
            .insert(node.range.start(), self.current_scope);
        self.generic_visit_expr_call(node);
    }

    fn visit_stmt_ann_assign(&mut self, node: ast::StmtAnnAssign) {
        if let ast::Expr::Name(target) = node.target.as_ref() {
            self.bind(target.id.as_str());
        }
        self.generic_visit_stmt_ann_assign(node);
    }

    fn visit_stmt_assign(&mut self, node: ast::StmtAssign) {
        for target in &node.targets {
            if let ast::Expr::Name(name) = target {
                self.bind(name.id.as_str());
            }
        }
        self.generic_visit_stmt_assign(node);
    }

    fn visit_stmt_async_function_def(&mut self, node: ast::StmtAsyncFunctionDef) {
        self.bind(node.name.as_str());
        self.visit_function_outer(
            &node.args,
            &node.decorator_list,
            node.returns.as_deref(),
            &node.type_params,
        );
        let parent = self.enter_scope();
        self.bind_arguments(&node.args);
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }

    fn visit_stmt_class_def(&mut self, node: ast::StmtClassDef) {
        self.bind(node.name.as_str());
        for base in &node.bases {
            self.visit_expr(base.clone());
        }
        for keyword in &node.keywords {
            self.visit_keyword(keyword.clone());
        }
        for decorator in &node.decorator_list {
            self.visit_expr(decorator.clone());
        }
        for type_param in &node.type_params {
            self.visit_type_param(type_param.clone());
        }
        let parent = self.enter_scope();
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }

    fn visit_stmt_function_def(&mut self, node: ast::StmtFunctionDef) {
        self.bind(node.name.as_str());
        self.visit_function_outer(
            &node.args,
            &node.decorator_list,
            node.returns.as_deref(),
            &node.type_params,
        );
        let parent = self.enter_scope();
        self.bind_arguments(&node.args);
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }

    fn visit_stmt_import(&mut self, node: ast::StmtImport) {
        for alias in &node.names {
            self.bind(import_bound_name(alias));
        }
        self.generic_visit_stmt_import(node);
    }

    fn visit_stmt_import_from(&mut self, node: ast::StmtImportFrom) {
        for alias in &node.names {
            if alias.name.as_str() == "*" {
                continue;
            }
            self.bind(import_bound_name(alias));
        }
        self.generic_visit_stmt_import_from(node);
    }
}

fn match_builtin_expr(
    expression: &ast::Expr,
    builtin: &str,
    scope: usize,
    bindings: &Bindings,
) -> bool {
    match expression {
        ast::Expr::Name(name) => {
            name.id.as_str() == builtin && bindings.is_unbound(name.id.as_str(), scope)
        }
        ast::Expr::Attribute(_) => bare_or_builtins_attr(expression, builtin),
        _ => false,
    }
}

fn bare_or_builtins_attr(expression: &ast::Expr, builtin: &str) -> bool {
    match expression {
        ast::Expr::Name(name) => name.id.as_str() == builtin,
        ast::Expr::Attribute(attribute) => {
            attribute.attr.as_str() == builtin
                && matches!(
                    attribute.value.as_ref(),
                    ast::Expr::Name(name) if name.id.as_str() == "builtins"
                )
        }
        _ => false,
    }
}

fn import_bound_name(alias: &ast::Alias) -> &str {
    if let Some(name) = &alias.asname {
        return name.as_str();
    }
    alias.name.split('.').next().unwrap_or(alias.name.as_str())
}

#[cfg(test)]
mod tests {
    use rustpython_parser::Parse;
    use rustpython_parser::ast::{self, Visitor};

    use super::Bindings;

    fn match_builtin(source: &str, builtin: &str) -> bool {
        let module = ast::Suite::parse(source, "<test>").expect("parse");
        let bindings = Bindings::from_module(&module);
        let mut calls = Calls::default();
        for statement in module {
            calls.visit_stmt(statement);
        }
        bindings.match_builtin_expr(calls.items.last().expect("call"), builtin)
    }

    #[derive(Default)]
    struct Calls {
        items: Vec<ast::ExprCall>,
    }

    impl Visitor for Calls {
        fn visit_expr_call(&mut self, node: ast::ExprCall) {
            self.items.push(node.clone());
            self.generic_visit_expr_call(node);
        }
    }

    #[test]
    fn unbound_isinstance_matches_builtin() {
        assert!(match_builtin("isinstance(1, int)\n", "isinstance"));
    }

    #[test]
    fn builtins_attr_matches_builtin() {
        assert!(match_builtin("builtins.isinstance(1, int)\n", "isinstance"));
    }

    #[test]
    fn shadowed_isinstance_does_not_match_builtin() {
        assert!(!match_builtin(
            "isinstance = check\nisinstance(1, int)\n",
            "isinstance"
        ));
    }

    #[test]
    fn imported_isinstance_does_not_match_builtin() {
        assert!(!match_builtin(
            "from helpers import isinstance\nisinstance(1, int)\n",
            "isinstance"
        ));
    }
}
