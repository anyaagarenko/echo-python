use std::path::Path;

use echo_python::{CheckOptions, Diagnostic, RULE_CALLS_USE_KWARGS, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

fn lint_with(source: &str, options: &CheckOptions) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, options).expect("lint")
}

#[test]
fn single_positional_is_clean() {
    assert!(lint("f(1)\n").is_empty());
}

#[test]
fn kwargs_only_is_clean() {
    assert!(lint("f(a=1, b=2)\n").is_empty());
}

#[test]
fn two_positionals_are_reported() {
    let diags = lint("f(1, 2)\n");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_CALLS_USE_KWARGS, diags[0].code);
}

#[test]
fn method_two_positionals_are_reported() {
    assert_eq!(RULE_CALLS_USE_KWARGS, lint("obj.m(1, 2)\n")[0].code);
}

#[test]
fn dict_get_is_clean() {
    assert!(lint("data = {}\ndata.get(\"key\", None)\n").is_empty());
}

#[test]
fn dict_pop_is_clean() {
    assert!(lint("data: dict = {}\ndata.pop(\"key\", None)\n").is_empty());
}

#[test]
fn unknown_receiver_get_is_clean() {
    assert!(lint("repository.get(\"key\", None)\n").is_empty());
}

#[test]
fn unknown_receiver_pop_is_clean() {
    assert!(lint("queue.pop(\"key\", None)\n").is_empty());
}

#[test]
fn setdefault_is_clean() {
    assert!(lint("kwargs.setdefault(\"timeout\", 1)\n").is_empty());
}

#[test]
fn decorator_uses_outer_scope() {
    let source = "repository = object()\n@decorate(repository.fetch(\"key\", None))\ndef f(repository: dict):\n    pass\n";
    assert_eq!(RULE_CALLS_USE_KWARGS, lint(source)[0].code);
}

#[test]
fn other_method_is_reported() {
    assert_eq!(
        RULE_CALLS_USE_KWARGS,
        lint("helper.combine(\"a\", \"b\")\n")[0].code
    );
}

#[test]
fn kwargs_get_is_clean() {
    assert!(lint("def f(**options):\n    options.get(\"key\", None)\n").is_empty());
}

#[test]
fn annotated_class_dict_get_is_clean() {
    let source = "class C:\n    data: dict | None = None\n    def f(self):\n        self.data.get(\"key\", None)\n";
    assert!(lint(source).is_empty());
}

#[test]
fn round_is_clean() {
    assert!(lint("round(1.5, 2)\n").is_empty());
}

#[test]
fn int_is_clean() {
    assert!(lint("int(\"1\", 10)\n").is_empty());
}

#[test]
fn self_then_one_arg_is_clean() {
    assert!(lint("Foo.m(self, x)\n").is_empty());
}

#[test]
fn self_then_two_args_are_reported() {
    assert_eq!(RULE_CALLS_USE_KWARGS, lint("Foo.m(self, x, y)\n")[0].code);
}

#[test]
fn starred_with_one_positional_is_clean() {
    assert!(lint("f(1, *xs)\n").is_empty());
}

#[test]
fn noqa_suppresses() {
    assert!(lint("f(1, 2)  # noqa: ECHO001\n").is_empty());
}

#[test]
fn isinstance_is_clean() {
    assert!(lint("isinstance(1, int)\n").is_empty());
}

#[test]
fn builtins_isinstance_is_clean() {
    assert!(lint("builtins.isinstance(1, int)\n").is_empty());
}

#[test]
fn shadowed_isinstance_is_reported() {
    assert_eq!(
        RULE_CALLS_USE_KWARGS,
        lint("isinstance = check\nisinstance(1, int)\n")[0].code
    );
}

#[test]
fn select_only_other_rule_skips() {
    let options = CheckOptions {
        select: Some(vec!["ECHO003".into()]),
        ..CheckOptions::default()
    };
    assert!(lint_with("f(1, 2)\n", &options).is_empty());
}

#[test]
fn ignore_rule_skips() {
    let options = CheckOptions {
        ignore: vec!["ECHO001".into()],
        ..CheckOptions::default()
    };
    assert!(lint_with("f(1, 2)\n", &options).is_empty());
}

#[test]
fn ignore_from_pyproject() {
    let root =
        std::env::temp_dir().join(format!("echo-python-kwargs-ignore-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("pyproject.toml"),
        "[tool.echo-python.calls-use-kwargs]\nignore = [\"print\"]\n",
    )
    .unwrap();
    let file = root.join("t.py");
    let source = "print(1, 2)\n";
    std::fs::write(&file, source).unwrap();
    let diags = check_source(&file, source, &CheckOptions::default()).expect("lint");
    assert!(diags.is_empty());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn qualified_ignore_from_pyproject() {
    let root = std::env::temp_dir().join(format!(
        "echo-python-kwargs-qualified-ignore-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("pyproject.toml"),
        "[tool.echo-python.calls-use-kwargs]\nignore = [\"pytest.param\"]\n",
    )
    .unwrap();
    let file = root.join("t.py");
    let source = "pytest.param(1, 2, id=\"x\")\nother.param(1, 2)\n";
    std::fs::write(&file, source).unwrap();
    let diags = check_source(&file, source, &CheckOptions::default()).expect("lint");
    assert_eq!(1, diags.len());
    assert_eq!(RULE_CALLS_USE_KWARGS, diags[0].code);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn lint_ignore_from_pyproject() {
    let root = std::env::temp_dir().join(format!("echo-python-lint-ignore-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("pyproject.toml"),
        "[tool.echo-python.lint]\nignore = [\"ECHO001\"]\n",
    )
    .unwrap();
    let file = root.join("t.py");
    let source = "f(1, 2)\n";
    std::fs::write(&file, source).unwrap();
    let diags = check_source(&file, source, &CheckOptions::default()).expect("lint");
    assert!(diags.is_empty());
    let _ = std::fs::remove_dir_all(&root);
}
