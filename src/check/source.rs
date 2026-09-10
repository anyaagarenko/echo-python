use std::path::Path;

use anyhow::{Context, Result};
use rustpython_parser::Parse;
use rustpython_parser::ast::{self, Visitor};

use crate::bindings::Bindings;
use crate::checker::Checker;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::{self, CheckOptions, Settings};

pub(super) fn check_file(path: &Path, options: &CheckOptions) -> Result<Vec<Diagnostic>> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    check_source(path, &source, options)
}

pub fn check_source(path: &Path, source: &str, options: &CheckOptions) -> Result<Vec<Diagnostic>> {
    let module = parse_module(path, source)?;
    let locator = Locator::new(source);
    let noqa = NoqaIndex::from_source(source);
    let settings = settings::load_for_path(path, options);
    let bindings = Bindings::from_module(&module);
    let mut diagnostics = Vec::new();
    visit_module(
        path,
        &bindings,
        &locator,
        &noqa,
        &settings,
        module,
        &mut diagnostics,
    );
    Ok(diagnostics)
}

fn parse_module(path: &Path, source: &str) -> Result<ast::Suite> {
    ast::Suite::parse(source, &path.display().to_string())
        .map_err(|err| anyhow::anyhow!("{}: parse error: {err}", path.display()))
}

fn visit_module(
    path: &Path,
    bindings: &Bindings,
    locator: &Locator,
    noqa: &NoqaIndex,
    settings: &Settings,
    module: ast::Suite,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut checker = Checker {
        bindings,
        diagnostics,
        locator,
        noqa,
        path,
        settings,
    };
    for stmt in module {
        checker.visit_stmt(stmt);
    }
}
