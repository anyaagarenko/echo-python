use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

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
    #[serde(rename = "calls-use-kwargs")]
    calls_use_kwargs: Option<CallsUseKwargsTable>,
}

#[derive(Debug, Default, Deserialize)]
struct CallsUseKwargsTable {
    #[serde(default)]
    ignore: Vec<String>,
}

pub(crate) fn load_for_path(path: &Path) -> Settings {
    let Some(pyproject) = find_pyproject(path) else {
        return Settings::default();
    };
    load_from_file(&pyproject).unwrap_or_default()
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

fn load_from_file(path: &Path) -> Option<Settings> {
    let source = fs::read_to_string(path).ok()?;
    parse_pyproject(&source)
}

fn parse_pyproject(source: &str) -> Option<Settings> {
    let parsed: PyProject = toml::from_str(source).ok()?;
    let table = parsed
        .tool
        .and_then(|tool| tool.echo_python)
        .and_then(|echo| echo.calls_use_kwargs)
        .unwrap_or_default();
    Some(Settings {
        calls_use_kwargs: CallsUseKwargsSettings {
            ignore: table.ignore.into_iter().collect(),
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
    fn missing_section_is_empty() {
        let settings = parse_pyproject("[project]\nname = \"x\"\n").unwrap();
        assert!(settings.calls_use_kwargs.ignore.is_empty());
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

    fn tempfile_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("echo-python-settings-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
