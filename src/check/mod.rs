mod paths;
mod source;
mod walk;

use crate::diagnostic::Diagnostic;

pub use paths::{check_path, check_paths};
pub use source::check_source;

#[derive(Debug, Default)]
pub struct CheckResult {
    pub diagnostics: Vec<Diagnostic>,
}
