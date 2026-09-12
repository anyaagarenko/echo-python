use rustpython_parser::ast;

pub(super) fn is_queryset_call(expr: &ast::ExprCall) -> bool {
    let ast::Expr::Attribute(attribute) = expr.func.as_ref() else {
        return false;
    };
    let method = attribute.attr.as_str();
    if is_distinctive_queryset_method(method) {
        return true;
    }
    is_ambiguous_queryset_method(method) && is_queryset_receiver(attribute.value.as_ref())
}

fn is_queryset_receiver(expr: &ast::Expr) -> bool {
    match expr {
        ast::Expr::Attribute(attribute) if attribute.attr.as_str() == "objects" => true,
        ast::Expr::Call(call) => is_queryset_call(call),
        _ => false,
    }
}

fn is_distinctive_queryset_method(method: &str) -> bool {
    matches!(
        method,
        "alias"
            | "annotate"
            | "dates"
            | "datetimes"
            | "defer"
            | "only"
            | "prefetch_related"
            | "select_for_update"
            | "select_related"
            | "values"
            | "values_list"
    )
}

fn is_ambiguous_queryset_method(method: &str) -> bool {
    matches!(
        method,
        "aggregate"
            | "all"
            | "create"
            | "difference"
            | "distinct"
            | "exclude"
            | "filter"
            | "get"
            | "intersection"
            | "none"
            | "order_by"
            | "reverse"
            | "union"
            | "update"
    )
}
