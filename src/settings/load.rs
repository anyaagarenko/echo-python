use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::rules::{CheckOptions, LintFileSettings, resolve_enabled};
use super::{Echo001Settings, Echo006Settings, Echo007Settings, Settings};

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
    echo006: Option<Echo006Table>,
    echo007: Option<Echo007Table>,
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

#[derive(Debug, Default, Deserialize)]
struct Echo006Table {
    #[serde(default)]
    names: Vec<String>,
    #[serde(default)]
    allow_msg: bool,
}

#[derive(Debug, Deserialize)]
struct Echo007Table {
    #[serde(default = "default_true")]
    exclude_tests: bool,
}

impl Default for Echo007Table {
    fn default() -> Self {
        Self {
            exclude_tests: true,
        }
    }
}

const fn default_true() -> bool {
    true
}

pub(crate) fn load_for_path(path: &Path, options: &CheckOptions) -> Settings {
    let file = find_pyproject(path)
        .and_then(|pyproject| load_file_settings(&pyproject))
        .unwrap_or_default();
    Settings {
        enabled: resolve_enabled(&file.lint, options),
        echo001: file.echo001,
        echo006: file.echo006,
        echo007: file.echo007,
    }
}

#[derive(Debug, Default)]
struct FileSettings {
    lint: LintFileSettings,
    echo001: Echo001Settings,
    echo006: Echo006Settings,
    echo007: Echo007Settings,
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
    let echo006 = echo.echo006.unwrap_or_default();
    let echo007 = echo.echo007.unwrap_or_default();
    Some(FileSettings {
        lint: LintFileSettings {
            select: lint.select,
            extend_select: lint.extend_select,
            ignore: lint.ignore,
        },
        echo001: Echo001Settings {
            ignore: echo001.ignore.into_iter().collect(),
        },
        echo006: Echo006Settings::from_config(echo006.names, echo006.allow_msg),
        echo007: Echo007Settings {
            exclude_tests: echo007.exclude_tests,
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

    #[test]
    fn echo006_defaults_to_msg() {
        let source = "[tool.echo-python.lint]\nselect = [\"ALL\"]\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(settings.echo006.is_restricted("msg"));
        assert!(!settings.echo006.is_restricted("err"));
    }

    #[test]
    fn parses_echo006_names_as_extra() {
        let source = "[tool.echo-python.echo006]\nnames = [\"err\"]\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(settings.echo006.is_restricted("msg"));
        assert!(settings.echo006.is_restricted("err"));
    }

    #[test]
    fn allow_msg_unmutes_default() {
        let source = "[tool.echo-python.echo006]\nallow_msg = true\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(!settings.echo006.is_restricted("msg"));
    }

    #[test]
    fn allow_msg_keeps_extra_names() {
        let source = "[tool.echo-python.echo006]\nnames = [\"err\"]\nallow_msg = true\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(!settings.echo006.is_restricted("msg"));
        assert!(settings.echo006.is_restricted("err"));
    }

    #[test]
    fn echo007_defaults_to_exclude_tests() {
        let source = "[tool.echo-python.lint]\nselect = [\"ALL\"]\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(settings.echo007.exclude_tests);
        assert!(settings.echo007.skips_function("test_f"));
    }

    #[test]
    fn parses_echo007_exclude_tests() {
        let source = "[tool.echo-python.echo007]\nexclude_tests = false\n";

        let settings = parse_pyproject(source).unwrap();

        assert!(!settings.echo007.exclude_tests);
        assert!(!settings.echo007.skips_function("test_f"));
    }
}
