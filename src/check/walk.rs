use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

pub(super) fn collect_python_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        collect_from_path(path, &mut files)?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_from_path(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !path.exists() {
        bail!("path does not exist: {}", path.display());
    }
    if path.is_file() {
        if is_python_file(path) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    collect_from_dir(path, files)
}

fn collect_from_dir(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !is_excluded_dir(e.path()))
    {
        let entry = entry.with_context(|| format!("failed walking {}", path.display()))?;
        if entry.file_type().is_file() && is_python_file(entry.path()) {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_python_extension() {
        assert!(is_python_file(Path::new("a.py")));
    }

    #[test]
    fn rejects_non_python_extension() {
        assert!(!is_python_file(Path::new("a.rs")));
    }

    #[test]
    fn excludes_git_dir() {
        assert!(is_excluded_dir(Path::new(".git")));
    }

    #[test]
    fn excludes_pycache_dir() {
        assert!(is_excluded_dir(Path::new("__pycache__")));
    }
}
