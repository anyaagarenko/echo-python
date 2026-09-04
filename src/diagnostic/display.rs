use std::fmt;
use std::path::Path;

use super::Diagnostic;

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
