use std::path::PathBuf;

use super::Diagnostic;

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
