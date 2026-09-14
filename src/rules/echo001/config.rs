use rustpython_parser::ast;

use crate::settings::Settings;

pub(super) fn is_ignored(settings: &Settings, func: &ast::Expr) -> bool {
    let Some(path) = callee_path(func) else {
        return false;
    };
    settings.echo001.ignores(path.as_str()) || settings.echo001.ignores(&short_name(&path))
}

pub(super) fn callee_name(func: &ast::Expr) -> Option<String> {
    callee_path(func).map(|path| short_name(&path))
}

fn short_name(path: &str) -> String {
    path.rsplit('.').next().unwrap_or(path).to_string()
}

fn callee_path(func: &ast::Expr) -> Option<String> {
    match func {
        ast::Expr::Name(name) => Some(name.id.to_string()),
        ast::Expr::Attribute(attr) => Some(callee_path(attr.value.as_ref()).map_or_else(
            || attr.attr.to_string(),
            |base| format!("{base}.{}", attr.attr.as_str()),
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::Suite;

    fn func_from(source: &str) -> ast::Expr {
        let module = Suite::parse(source, "<test>").expect("parse");
        match module.into_iter().next().expect("stmt") {
            ast::Stmt::Expr(stmt) => match *stmt.value {
                ast::Expr::Call(call) => *call.func,
                other => panic!("expected call, got {other:?}"),
            },
            other => panic!("expected expr stmt, got {other:?}"),
        }
    }

    #[test]
    fn callee_path_from_attribute() {
        assert_eq!(
            Some("obj.append"),
            callee_path(&func_from("obj.append(1, 2)\n")).as_deref()
        );
    }

    #[test]
    fn callee_name_uses_short_method() {
        assert_eq!(
            Some("replace"),
            callee_name(&func_from("name.replace(\"a\", \"b\")\n")).as_deref()
        );
    }

    #[test]
    fn callee_path_from_nested_attribute() {
        assert_eq!(
            Some("pytest.param"),
            callee_path(&func_from("pytest.param(1, 2)\n")).as_deref()
        );
    }

    #[test]
    fn callee_path_falls_back_on_call_chain() {
        assert_eq!(
            Some("values_list"),
            callee_path(&func_from("qs.filter(x=1).values_list(\"a\", \"b\")\n")).as_deref()
        );
    }
}
