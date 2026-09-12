use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::rules::{CheckOptions, LintFileSettings, resolve_enabled};
use super::{Echo001Settings, Settings};

#[derive(Debug, Default, Deserialize)]
struct PyProject {
    tool: Option<ToolTable>,
}

#[derive(Debug, Default, Deserialize)]
struct ToolTable {
    #[serde(rename = "echo-python")]
    echo_python: Option<EchoPythonTable>,
}

#[derive(Debug, Default, Deserialize)]
struct EchoPythonTable {
    lint: Option<LintTable>,
    echo001: Option<Echo001Table>,
}

#[derive(Debug, Default, Deserialize)]
struct LintTable {
    select: Option<Vec<String>>,
    #[serde(default, rename = "extend-select")]
    extend_select: Vec<String>,
    #[serde(default)]
    ignore: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct Echo001Table {
    #[serde(default)]
    ignore: Vec<String>,
}

pub(crate) fn load_for_path(path: &Path, options: &CheckOptions) -> Settings {
    let file = find_pyproject(path)
        .and_then(|pyproject| load_file_settings(&pyproject))
        .unwrap_or_default();
    Settings {
        enabled: resolve_enabled(&file.lint, options),
        echo001: file.echo001,
    }
}

#[derive(Debug, Default)]
struct FileSettings {
    lint: LintFileSettings,
    echo001: Echo001Settings,
}

fn find_pyproject(path: &Path) -> Option<PathBuf> {
    let mut dir = if path.is_file() {
        path.parent()?.to_path_buf()
    } else {
        path.to_path_buf()
    };
    loop {
        let candidate = dir.join("pyproject.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn load_file_settings(path: &Path) -> Option<FileSettings> {
    let source = fs::read_to_string(path).ok()?;
    parse_pyproject(&source)
}

fn parse_pyproject(source: &str) -> Option<FileSettings> {
    let parsed: PyProject = toml::from_str(source).ok()?;
    let echo = parsed.tool.and_then(|tool| tool.echo_python)?;
    let lint = echo.lint.unwrap_or_default();
    let echo001 = echo.echo001.unwrap_or_default();
    Some(FileSettings {
        lint: LintFileSettings {
            select: lint.select,
            extend_select: lint.extend_select,
            ignore: lint.ignore,
        },
        echo001: Echo001Settings {
            ignore: echo001.ignore.into_iter().collect(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tempfile_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "echo-python-settings-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn parses_ignore_list() {
        let source = "[tool.echo-python.echo001]\nignore = [\"print\", \"len\"]\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(settings.echo001.ignores("print"));
        assert!(settings.echo001.ignores("len"));
        assert!(!settings.echo001.ignores("foo"));
    }

    #[test]
    fn parses_lint_select_and_ignore() {
        let source = "[tool.echo-python.lint]\nselect = [\"ECHO003\"]\nignore = [\"ECHO001\"]\n";

        let file = parse_pyproject(source).unwrap();

        assert_eq!(Some(vec!["ECHO003".to_string()]), file.lint.select);
        assert_eq!(vec!["ECHO001".to_string()], file.lint.ignore);
    }

    #[test]
    fn missing_section_is_empty() {
        let settings = parse_pyproject("[project]\nname = \"x\"\n");

        assert!(settings.is_none());
    }

    #[test]
    fn find_pyproject_walks_up() {
        let root = tempfile_dir();
        fs::write(root.join("pyproject.toml"), "[project]\nname = \"x\"\n").unwrap();
        let nested = root.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();
        let file = nested.join("t.py");
        fs::write(&file, "x = 1\n").unwrap();

        let found = find_pyproject(&file).unwrap();

        assert_eq!(root.join("pyproject.toml"), found);
    }

    #[test]
    fn load_for_path_defaults_to_all_rules() {
        let root = tempfile_dir();
        let file = root.join("t.py");
        fs::write(&file, "x = 1\n").unwrap();

        let settings = load_for_path(&file, &CheckOptions::default());

        assert!(settings.is_enabled("ECHO003"));
        assert!(settings.is_enabled("ECHO001"));
    }
}
