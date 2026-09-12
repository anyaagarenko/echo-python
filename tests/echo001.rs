use std::path::{Path, PathBuf};

use echo_python::{CheckOptions, Diagnostic, RULE_ECHO001, check_source};

fn lint(source: &str) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, &CheckOptions::default()).expect("lint")
}

fn lint_with(source: &str, options: &CheckOptions) -> Vec<Diagnostic> {
    check_source(Path::new("t.py"), source, options).expect("lint")
}

fn lint_project(pyproject: &str, source: &str) -> Vec<Diagnostic> {
    let root = tempfile_dir("echo-python-echo001");
    std::fs::write(root.join("pyproject.toml"), pyproject).unwrap();
    let file = root.join("t.py");
    std::fs::write(&file, source).unwrap();
    let diags = check_source(&file, source, &CheckOptions::default()).expect("lint");
    let _ = std::fs::remove_dir_all(&root);
    diags
}

fn tempfile_dir(prefix: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn single_positional_is_clean() {
    let diags = lint("f(1)\n");

    assert!(diags.is_empty());
}

#[test]
fn kwargs_only_is_clean() {
    let diags = lint("f(a=1, b=2)\n");

    assert!(diags.is_empty());
}

#[test]
fn two_positionals_are_reported() {
    let diags = lint("f(1, 2)\n");

    assert_eq!(1, diags.len());
    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn method_two_positionals_are_reported() {
    let diags = lint("obj.m(1, 2)\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn dict_get_is_clean() {
    let diags = lint("data = {}\ndata.get(\"key\", None)\n");

    assert!(diags.is_empty());
}

#[test]
fn dict_pop_is_clean() {
    let diags = lint("data: dict = {}\ndata.pop(\"key\", None)\n");

    assert!(diags.is_empty());
}

#[test]
fn unknown_receiver_get_is_clean() {
    let diags = lint("repository.get(\"key\", None)\n");

    assert!(diags.is_empty());
}

#[test]
fn unknown_receiver_pop_is_clean() {
    let diags = lint("queue.pop(\"key\", None)\n");

    assert!(diags.is_empty());
}

#[test]
fn setdefault_is_clean() {
    let diags = lint("kwargs.setdefault(\"timeout\", 1)\n");

    assert!(diags.is_empty());
}

#[test]
fn decorator_uses_outer_scope() {
    let source = "repository = object()\n@decorate(repository.fetch(\"key\", None))\ndef f(repository: dict):\n    pass\n";

    let diags = lint(source);

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn other_method_is_reported() {
    let diags = lint("helper.combine(\"a\", \"b\")\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn kwargs_get_is_clean() {
    let diags = lint("def f(**options):\n    options.get(\"key\", None)\n");

    assert!(diags.is_empty());
}

#[test]
fn annotated_class_dict_get_is_clean() {
    let source = "class C:\n    data: dict | None = None\n    def f(self):\n        self.data.get(\"key\", None)\n";

    let diags = lint(source);

    assert!(diags.is_empty());
}

#[test]
fn round_is_reported() {
    let diags = lint("round(1.5, 2)\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn int_is_reported() {
    let diags = lint("int(\"1\", 10)\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn open_is_reported() {
    let diags = lint("open(\"a\", \"r\")\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn print_is_clean() {
    let diags = lint("print(1, 2)\n");

    assert!(diags.is_empty());
}

#[test]
fn self_then_one_arg_is_clean() {
    let diags = lint("Foo.m(self, x)\n");

    assert!(diags.is_empty());
}

#[test]
fn self_then_two_args_are_reported() {
    let diags = lint("Foo.m(self, x, y)\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn starred_with_one_positional_is_clean() {
    let diags = lint("f(1, *xs)\n");

    assert!(diags.is_empty());
}

#[test]
fn noqa_suppresses() {
    let diags = lint("f(1, 2)  # noqa: ECHO001\n");

    assert!(diags.is_empty());
}

#[test]
fn isinstance_is_clean() {
    let diags = lint("isinstance(1, int)\n");

    assert!(diags.is_empty());
}

#[test]
fn builtins_isinstance_is_clean() {
    let diags = lint("builtins.isinstance(1, int)\n");

    assert!(diags.is_empty());
}

#[test]
fn shadowed_isinstance_is_reported() {
    let diags = lint("isinstance = check\nisinstance(1, int)\n");

    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn select_only_other_rule_skips() {
    let options = CheckOptions {
        select: Some(vec!["ECHO003".into()]),
        ..CheckOptions::default()
    };

    let diags = lint_with("f(1, 2)\n", &options);

    assert!(diags.is_empty());
}

#[test]
fn ignore_rule_skips() {
    let options = CheckOptions {
        ignore: vec!["ECHO001".into()],
        ..CheckOptions::default()
    };

    let diags = lint_with("f(1, 2)\n", &options);

    assert!(diags.is_empty());
}

#[test]
fn ignore_from_pyproject() {
    let pyproject = "[tool.echo-python.echo001]\nignore = [\"helper\"]\n";

    let diags = lint_project(pyproject, "helper(1, 2)\n");

    assert!(diags.is_empty());
}

#[test]
fn qualified_ignore_from_pyproject() {
    let pyproject = "[tool.echo-python.echo001]\nignore = [\"pytest.param\"]\n";
    let source = "pytest.param(1, 2, id=\"x\")\nother.param(1, 2)\n";

    let diags = lint_project(pyproject, source);

    assert_eq!(1, diags.len());
    assert_eq!(RULE_ECHO001, diags[0].code);
}

#[test]
fn lint_ignore_from_pyproject() {
    let pyproject = "[tool.echo-python.lint]\nignore = [\"ECHO001\"]\n";

    let diags = lint_project(pyproject, "f(1, 2)\n");

    assert!(diags.is_empty());
}

#[test]
fn objects_filter_is_clean() {
    let diags = lint("User.objects.filter(\"a\", \"b\")\n");

    assert!(diags.is_empty());
}

#[test]
fn chained_queryset_order_by_is_clean() {
    let diags = lint("User.objects.all().order_by(\"a\", \"-b\")\n");

    assert!(diags.is_empty());
}

#[test]
fn select_related_is_clean() {
    let diags = lint("qs.select_related(\"a\", \"b\")\n");

    assert!(diags.is_empty());
}

#[test]
fn filter_method_is_clean() {
    let diags = lint("helper.filter(\"a\", \"b\")\n");

    assert!(diags.is_empty());
}
