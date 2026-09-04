use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use rustpython_parser::Parse;
use rustpython_parser::ast::{self, Visitor};
use walkdir::WalkDir;

use crate::checker::Checker;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

#[derive(Debug, Default)]
pub struct CheckResult {
    pub diagnostics: Vec<Diagnostic>,
}

pub fn check_paths(paths: &[PathBuf]) -> Result<CheckResult> {
    let files = collect_python_files(paths)?;
    let chunks: Vec<Result<Vec<Diagnostic>>> =
        files.par_iter().map(|path| check_file(path)).collect();

    let mut diagnostics = Vec::new();
    for chunk in chunks {
        diagnostics.extend(chunk?);
    }
    diagnostics.sort_by(|a, b| {
        (&a.path, a.row, a.column, a.code).cmp(&(&b.path, b.row, b.column, b.code))
    });

    Ok(CheckResult { diagnostics })
}

pub fn check_path(path: impl AsRef<Path>) -> Result<CheckResult> {
    check_paths(&[path.as_ref().to_path_buf()])
}

fn check_file(path: &Path) -> Result<Vec<Diagnostic>> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    check_source(path, &source)
}

pub fn check_source(path: &Path, source: &str) -> Result<Vec<Diagnostic>> {
    let module = ast::Suite::parse(source, &path.display().to_string())
        .map_err(|err| anyhow::anyhow!("{}: parse error: {err}", path.display()))?;

    let locator = Locator::new(source);
    let noqa = NoqaIndex::from_source(source);
    let mut diagnostics = Vec::new();

    let mut checker = Checker {
        locator: &locator,
        noqa: &noqa,
        path,
        diagnostics: &mut diagnostics,
    };

    for stmt in module {
        checker.visit_stmt(stmt);
    }

    Ok(diagnostics)
}

fn collect_python_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for path in paths {
        if !path.exists() {
            bail!("path does not exist: {}", path.display());
        }

        if path.is_file() {
            if is_python_file(path) {
                files.push(path.clone());
            }
            continue;
        }

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| e.depth() == 0 || !is_excluded_dir(e.path()))
        {
            let entry = entry.with_context(|| format!("failed walking {}", path.display()))?;
            if entry.file_type().is_file() && is_python_file(entry.path()) {
                files.push(entry.path().to_path_buf());
            }
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn is_python_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("py")
}

fn is_excluded_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                ".git"
                    | ".hg"
                    | ".svn"
                    | ".tox"
                    | ".venv"
                    | "venv"
                    | "__pycache__"
                    | "node_modules"
            ) || (name.starts_with('.') && path.is_dir())
        })
}
