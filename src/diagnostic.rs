use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub row: usize,
    pub column: usize,
    pub code: &'static str,
    pub message: String,
}

impl Diagnostic {
    pub fn new(
        path: impl Into<PathBuf>,
        row: usize,
        column: usize,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            row,
            column,
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {} - {}",
            display_path(&self.path),
            self.row,
            self.column,
            self.code,
            self.message
        )
    }
}

fn display_path(path: &Path) -> String {
    path.display().to_string()
}
