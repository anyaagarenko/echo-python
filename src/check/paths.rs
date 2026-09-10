use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::prelude::*;

use super::CheckResult;
use super::source::check_file;
use super::walk::collect_python_files;
use crate::diagnostic::Diagnostic;
use crate::settings::CheckOptions;

pub fn check_paths(paths: &[PathBuf], options: &CheckOptions) -> Result<CheckResult> {
    let files = collect_python_files(paths)?;
    let chunks: Vec<Result<Vec<Diagnostic>>> = files
        .par_iter()
        .map(|path| check_file(path, options))
        .collect();

    let mut diagnostics = Vec::new();
    for chunk in chunks {
        diagnostics.extend(chunk?);
    }
    sort_diagnostics(&mut diagnostics);

    Ok(CheckResult { diagnostics })
}

pub fn check_path(path: impl AsRef<Path>, options: &CheckOptions) -> Result<CheckResult> {
    check_paths(&[path.as_ref().to_path_buf()], options)
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|a, b| {
        (&a.path, a.row, a.column, a.code).cmp(&(&b.path, b.row, b.column, b.code))
    });
}
