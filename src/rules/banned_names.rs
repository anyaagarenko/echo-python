use rustpython_parser::ast::Ranged;

use crate::RULE_BANNED_NAMES;
use crate::common::report::report;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

pub(crate) fn check(
    locator: &Locator,
    noqa: &NoqaIndex,
    path: &std::path::Path,
    settings: &Settings,
    name: &str,
    node: &impl Ranged,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !settings.echo006.is_restricted(name) {
        return;
    }
    report(
        locator,
        noqa,
        path,
        node,
        RULE_BANNED_NAMES,
        &format!("variable name `{name}` is restricted"),
        diagnostics,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Echo006Settings;
    use rustpython_parser::Parse;
    use rustpython_parser::ast::{self, Suite};
    use std::collections::HashSet;
    use std::path::Path;

    fn settings_with(names: &[&str]) -> Settings {
        Settings {
            enabled: HashSet::from([RULE_BANNED_NAMES.to_string()]),
            echo006: Echo006Settings {
                names: names.iter().map(|name| (*name).to_string()).collect(),
            },
            ..Settings::default()
        }
    }

    fn diagnose(source: &str, settings: &Settings) -> Vec<Diagnostic> {
        let module = Suite::parse(source, "<test>").expect("parse");
        let locator = Locator::new(source);
        let noqa = NoqaIndex::from_source(source);
        let mut diagnostics = Vec::new();
        for stmt in module {
            match stmt {
                ast::Stmt::Assign(node) => {
                    for target in &node.targets {
                        if let ast::Expr::Name(name) = target {
                            check(
                                &locator,
                                &noqa,
                                Path::new("t.py"),
                                settings,
                                name.id.as_str(),
                                name,
                                &mut diagnostics,
                            );
                        }
                    }
                }
                ast::Stmt::Try(node) => {
                    for handler in &node.handlers {
                        let ast::ExceptHandler::ExceptHandler(handler) = handler;
                        if let Some(name) = &handler.name {
                            check(
                                &locator,
                                &noqa,
                                Path::new("t.py"),
                                settings,
                                name.as_str(),
                                handler,
                                &mut diagnostics,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        diagnostics
    }

    #[test]
    fn restricted_assignment_is_reported() {
        assert_eq!(1, diagnose("msg = 1\n", &settings_with(&["msg"])).len());
    }

    #[test]
    fn other_assignment_is_clean() {
        assert!(diagnose("error = 1\n", &settings_with(&["msg"])).is_empty());
    }

    #[test]
    fn except_as_restricted_is_reported() {
        let source = "try:\n    pass\nexcept Exception as msg:\n    pass\n";
        assert_eq!(1, diagnose(source, &settings_with(&["msg"])).len());
    }
}
