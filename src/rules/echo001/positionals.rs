use rustpython_parser::ast;

pub(super) fn countable(args: &[ast::Expr]) -> usize {
    let args = skip_self_or_cls(args);
    args.iter().filter(|arg| !is_starred(arg)).count()
}

fn skip_self_or_cls(args: &[ast::Expr]) -> &[ast::Expr] {
    match args.first() {
        Some(arg) if is_self_or_cls(arg) => &args[1..],
        _ => args,
    }
}

fn is_self_or_cls(expr: &ast::Expr) -> bool {
    matches!(
        expr,
        ast::Expr::Name(name) if name.id.as_str() == "self" || name.id.as_str() == "cls"
    )
}

const fn is_starred(expr: &ast::Expr) -> bool {
    matches!(expr, ast::Expr::Starred(_))
}
