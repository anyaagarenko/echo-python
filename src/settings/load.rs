use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::rules::{CheckOptions, LintFileSettings, resolve_enabled};
use super::{CallsUseKwargsSettings, Settings};

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
    #[serde(rename = "calls-use-kwargs")]
    calls_use_kwargs: Option<CallsUseKwargsTable>,
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
struct CallsUseKwargsTable {
    #[serde(default)]
    ignore: Vec<String>,
}

pub(crate) fn load_for_path(path: &Path, options: &CheckOptions) -> Settings {
    let file = find_pyproject(path)
        .and_then(|pyproject| load_file_settings(&pyproject))
        .unwrap_or_default();
    Settings {
        enabled: resolve_enabled(&file.lint, options),
        calls_use_kwargs: file.calls_use_kwargs,
    }
}

#[derive(Debug, Default)]
struct FileSettings {
    lint: LintFileSettings,
    calls_use_kwargs: CallsUseKwargsSettings,
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
    let calls = echo.calls_use_kwargs.unwrap_or_default();
    Some(FileSettings {
        lint: LintFileSettings {
            select: lint.select,
            extend_select: lint.extend_select,
            ignore: lint.ignore,
        },
        calls_use_kwargs: CallsUseKwargsSettings {
            ignore: calls.ignore.into_iter().collect(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ignore_list() {
        let source = r#"
[tool.echo-python.calls-use-kwargs]
ignore = ["print", "len"]
"#;
        let settings = parse_pyproject(source).unwrap();
        assert!(settings.calls_use_kwargs.ignores("print"));
        assert!(settings.calls_use_kwargs.ignores("len"));
        assert!(!settings.calls_use_kwargs.ignores("foo"));
    }

    #[test]
    fn parses_lint_select_and_ignore() {
        let source = r#"
[tool.echo-python.lint]
select = ["ECHO003"]
ignore = ["ECHO001"]
"#;
        let file = parse_pyproject(source).unwrap();
        assert_eq!(Some(vec!["ECHO003".to_string()]), file.lint.select);
        assert_eq!(vec!["ECHO001".to_string()], file.lint.ignore);
    }

    #[test]
    fn missing_section_is_empty() {
        assert!(parse_pyproject("[project]\nname = \"x\"\n").is_none());
    }

    #[test]
    fn find_pyproject_walks_up() {
        let root = tempfile_dir();
        fs::write(root.join("pyproject.toml"), "[project]\nname = \"x\"\n").unwrap();
        let nested = root.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();
        let file = nested.join("t.py");
        fs::write(&file, "x = 1\n").unwrap();
        assert_eq!(root.join("pyproject.toml"), find_pyproject(&file).unwrap());
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
}
