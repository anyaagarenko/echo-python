use std::collections::HashMap;

use rustpython_parser::ast::{self, Visitor};
use rustpython_parser::text_size::TextSize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Binding {
    Dict,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScopeKind {
    Class,
    Function,
    Module,
}

#[derive(Debug)]
struct Scope {
    bindings: HashMap<String, Binding>,
    kind: ScopeKind,
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

    pub(crate) fn receiver_is_dict(&self, call: &ast::ExprCall) -> bool {
        let Some(scope) = self.call_scopes.get(&call.range.start()) else {
            return false;
        };
        let ast::Expr::Attribute(method) = call.func.as_ref() else {
            return false;
        };

        self.expression_is_dict(method.value.as_ref(), *scope)
    }

    fn expression_is_dict(&self, expression: &ast::Expr, scope: usize) -> bool {
        match expression {
            ast::Expr::Name(name) => self.lookup(name.id.as_str(), scope) == Some(Binding::Dict),
            ast::Expr::Attribute(attribute) => self.class_attribute_is_dict(attribute, scope),
            ast::Expr::Dict(_) => true,
            ast::Expr::Call(call) => is_dict_constructor(call),
            _ => false,
        }
    }

    fn class_attribute_is_dict(&self, attribute: &ast::ExprAttribute, scope: usize) -> bool {
        let ast::Expr::Name(owner) = attribute.value.as_ref() else {
            return false;
        };
        if !matches!(owner.id.as_str(), "cls" | "self") {
            return false;
        }
        let Some(class_scope) = self.ancestor(scope, ScopeKind::Class) else {
            return false;
        };

        self.scopes[class_scope]
            .bindings
            .get(attribute.attr.as_str())
            == Some(&Binding::Dict)
    }

    fn lookup(&self, name: &str, mut scope: usize) -> Option<Binding> {
        loop {
            if let Some(binding) = self.scopes[scope].bindings.get(name) {
                return Some(*binding);
            }
            scope = self.scopes[scope].parent?;
        }
    }

    fn ancestor(&self, mut scope: usize, kind: ScopeKind) -> Option<usize> {
        loop {
            if self.scopes[scope].kind == kind {
                return Some(scope);
            }
            scope = self.scopes[scope].parent?;
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
                    bindings: HashMap::new(),
                    kind: ScopeKind::Module,
                    parent: None,
                }],
            },
            current_scope: 0,
        }
    }

    fn enter_scope(&mut self, kind: ScopeKind) -> usize {
        let parent = self.current_scope;
        self.bindings.scopes.push(Scope {
            bindings: HashMap::new(),
            kind,
            parent: Some(parent),
        });
        self.current_scope = self.bindings.scopes.len() - 1;

        parent
    }

    const fn leave_scope(&mut self, parent: usize) {
        self.current_scope = parent;
    }

    fn bind(&mut self, name: &str, binding: Binding) {
        let bindings = &mut self.bindings.scopes[self.current_scope].bindings;
        bindings
            .entry(name.to_string())
            .and_modify(|current| {
                if *current != binding {
                    *current = Binding::Unknown;
                }
            })
            .or_insert(binding);
    }

    fn bind_arguments(&mut self, arguments: &ast::Arguments) {
        for argument in arguments
            .posonlyargs
            .iter()
            .chain(&arguments.args)
            .chain(&arguments.kwonlyargs)
        {
            self.bind_argument(&argument.def);
        }
        if let Some(argument) = &arguments.vararg {
            self.bind_argument(argument);
        }
        if let Some(argument) = &arguments.kwarg {
            self.bind(argument.arg.as_str(), Binding::Dict);
        }
    }

    fn bind_argument(&mut self, argument: &ast::Arg) {
        let binding = argument
            .annotation
            .as_deref()
            .filter(|annotation| is_dict_annotation(annotation))
            .map_or(Binding::Unknown, |_| Binding::Dict);
        self.bind(argument.arg.as_str(), binding);
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
            let binding = if is_dict_annotation(node.annotation.as_ref()) {
                Binding::Dict
            } else {
                Binding::Unknown
            };
            self.bind(target.id.as_str(), binding);
        }
        self.generic_visit_stmt_ann_assign(node);
    }

    fn visit_stmt_assign(&mut self, node: ast::StmtAssign) {
        let binding = if expression_creates_dict(node.value.as_ref()) {
            Binding::Dict
        } else {
            Binding::Unknown
        };
        for target in &node.targets {
            if let ast::Expr::Name(name) = target {
                self.bind(name.id.as_str(), binding);
            }
        }
        self.generic_visit_stmt_assign(node);
    }

    fn visit_stmt_async_function_def(&mut self, node: ast::StmtAsyncFunctionDef) {
        self.visit_function_outer(
            &node.args,
            &node.decorator_list,
            node.returns.as_deref(),
            &node.type_params,
        );
        let parent = self.enter_scope(ScopeKind::Function);
        self.bind_arguments(&node.args);
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }

    fn visit_stmt_class_def(&mut self, node: ast::StmtClassDef) {
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
        let parent = self.enter_scope(ScopeKind::Class);
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }

    fn visit_stmt_function_def(&mut self, node: ast::StmtFunctionDef) {
        self.visit_function_outer(
            &node.args,
            &node.decorator_list,
            node.returns.as_deref(),
            &node.type_params,
        );
        let parent = self.enter_scope(ScopeKind::Function);
        self.bind_arguments(&node.args);
        for statement in node.body {
            self.visit_stmt(statement);
        }
        self.leave_scope(parent);
    }
}

fn expression_creates_dict(expression: &ast::Expr) -> bool {
    matches!(expression, ast::Expr::Dict(_))
        || matches!(expression, ast::Expr::Call(call) if is_dict_constructor(call))
}

fn is_dict_constructor(call: &ast::ExprCall) -> bool {
    matches!(
        call.func.as_ref(),
        ast::Expr::Name(name) if name.id.as_str() == "dict"
    )
}

fn is_dict_annotation(annotation: &ast::Expr) -> bool {
    match annotation {
        ast::Expr::Name(name) => matches!(name.id.as_str(), "Dict" | "dict"),
        ast::Expr::Attribute(attribute) => {
            matches!(attribute.attr.as_str(), "Dict" | "dict")
        }
        ast::Expr::Subscript(subscript) => {
            is_dict_annotation_base(subscript.value.as_ref(), subscript.slice.as_ref())
        }
        ast::Expr::BinOp(binary) if binary.op == ast::Operator::BitOr => {
            is_optional_dict(binary.left.as_ref(), binary.right.as_ref())
        }
        _ => false,
    }
}

fn is_dict_annotation_base(base: &ast::Expr, slice: &ast::Expr) -> bool {
    is_dict_annotation(base) || (is_optional_name(base) && is_dict_annotation(slice))
}

fn is_optional_dict(left: &ast::Expr, right: &ast::Expr) -> bool {
    (is_dict_annotation(left) && is_none_annotation(right))
        || (is_none_annotation(left) && is_dict_annotation(right))
}

fn is_optional_name(expression: &ast::Expr) -> bool {
    matches!(
        expression,
        ast::Expr::Name(name) if name.id.as_str() == "Optional"
    ) || matches!(
        expression,
        ast::Expr::Attribute(attribute) if attribute.attr.as_str() == "Optional"
    )
}

fn is_none_annotation(expression: &ast::Expr) -> bool {
    matches!(
        expression,
        ast::Expr::Constant(constant) if constant.value == ast::Constant::None
    )
}

#[cfg(test)]
mod tests {
    use rustpython_parser::Parse;
    use rustpython_parser::ast::{self, Visitor};

    use super::Bindings;

    fn receiver_is_dict(source: &str) -> bool {
        let module = ast::Suite::parse(source, "<test>").expect("parse");
        let bindings = Bindings::from_module(&module);
        let mut calls = Calls::default();
        for statement in module {
            calls.visit_stmt(statement);
        }
        bindings.receiver_is_dict(calls.items.last().expect("call"))
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
    fn kwargs_is_dict() {
        assert!(receiver_is_dict(
            "def f(**options):\n    options.get(\"a\", None)\n"
        ));
    }

    #[test]
    fn annotated_argument_is_dict() {
        assert!(receiver_is_dict(
            "def f(data: dict[str, int]):\n    data.get(\"a\", None)\n"
        ));
    }

    #[test]
    fn dict_literal_is_dict() {
        assert!(receiver_is_dict("data = {}\ndata.get(\"a\", None)\n"));
    }

    #[test]
    fn annotated_class_attribute_is_dict() {
        assert!(receiver_is_dict(
            "class C:\n    data: dict | None = None\n    def f(self):\n        self.data.get(\"a\", None)\n"
        ));
    }

    #[test]
    fn unknown_receiver_is_not_dict() {
        assert!(!receiver_is_dict("event.get(\"a\", None)\n"));
    }

    #[test]
    fn conflicting_assignment_is_not_dict() {
        assert!(!receiver_is_dict(
            "data = {}\ndata = repository\ndata.get(\"a\", None)\n"
        ));
    }
}
