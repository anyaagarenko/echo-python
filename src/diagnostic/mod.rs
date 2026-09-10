mod ctors;
mod display;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub row: usize,
    pub column: usize,
    pub code: &'static str,
    pub message: String,
}
