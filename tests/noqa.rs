use echo_python::check_source;
use std::path::Path;

#[test]
fn bare_noqa_suppresses_all() {
    let diags = check_source(Path::new("t.py"), "x = [2, 1]  # noqa\n").expect("lint");
    assert!(diags.is_empty());
}

#[test]
fn specific_noqa_is_case_insensitive() {
    let diags = check_source(
        Path::new("t.py"),
        "x = [2, 1]  # noqa: ECHO-NUMBERS-LIST-SORTED\n",
    )
    .expect("lint");
    assert!(diags.is_empty());
}

#[test]
fn unrelated_comment_does_not_suppress() {
    let diags = check_source(Path::new("t.py"), "x = [2, 1]  # not a noqa\n").expect("lint");
    assert!(!diags.is_empty());
}

#[test]
fn multiple_codes() {
    let diags = check_source(
        Path::new("t.py"),
        "x = [2, 1]  # noqa: echo-numbers-list-sorted, other\n",
    )
    .expect("lint");
    assert!(diags.is_empty());
}
