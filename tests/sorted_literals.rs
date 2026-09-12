use std::path::Path;

use echo_python::{
    CheckOptions, Diagnostic, RULE_MIXED_LIST_SORTED, RULE_NUMBERS_LIST_SORTED,
    RULE_WORDS_LIST_SORTED, check_source,
};

fn lint(source: &str) -> Vec<Diagnostic> {
    let options = CheckOptions {
        select: Some(vec!["ECHO002".into(), "ECHO003".into(), "ECHO004".into()]),
        ..CheckOptions::default()
    };
    check_source(Path::new("t.py"), source, &options).expect("lint")
}

#[test]
fn sorted_number_list_is_clean() {
    let diags = lint("x = [4911, 100500]\n");

    assert!(diags.is_empty());
}

#[test]
fn unsorted_number_list_is_reported() {
    let diags = lint("x = [100500, 4911]\n");

    assert_eq!(1, diags.len());
    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_number_tuple_is_reported() {
    let diags = lint("x = (2, 1)\n");

    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_number_set_is_reported() {
    let diags = lint("x = {2, 1}\n");

    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_number_dict_keys_are_reported() {
    let diags = lint("x = {2: \"a\", 1: \"b\"}\n");

    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}

#[test]
fn sorted_word_list_is_clean() {
    let diags = lint("x = [\"a\", \"b\"]\n");

    assert!(diags.is_empty());
}

#[test]
fn unsorted_word_list_is_reported() {
    let diags = lint("x = [\"b\", \"a\"]\n");

    assert_eq!(RULE_WORDS_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_name_tuple_is_reported() {
    let diags = lint("x = (b, a)\n");

    assert_eq!(RULE_WORDS_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_word_dict_keys_are_reported() {
    let diags = lint("x = {\"b\": 1, \"a\": 2}\n");

    assert_eq!(RULE_WORDS_LIST_SORTED, diags[0].code);
}

#[test]
fn sorted_mixed_list_is_clean() {
    let diags = lint("x = [1, 2, \"a\", \"b\"]\n");

    assert!(diags.is_empty());
}

#[test]
fn unsorted_mixed_tuple_is_reported() {
    let diags = lint("x = (\"a\", 1)\n");

    assert_eq!(RULE_MIXED_LIST_SORTED, diags[0].code);
}

#[test]
fn unsorted_mixed_dict_keys_are_reported() {
    let diags = lint("x = {\"a\": 1, 1: \"b\"}\n");

    assert_eq!(RULE_MIXED_LIST_SORTED, diags[0].code);
}

#[test]
fn dict_with_unpack_is_skipped() {
    let diags = lint("x = {\"b\": 1, **y}\n");

    assert!(diags.is_empty());
}

#[test]
fn complex_expressions_are_skipped() {
    let diags = lint("x = [1 + 1, 0]\n");

    assert!(diags.is_empty());
}

#[test]
fn noqa_suppresses_numbers() {
    let diags = lint("x = [2, 1]  # noqa: ECHO003\n");

    assert!(diags.is_empty());
}

#[test]
fn noqa_suppresses_words() {
    let diags = lint("x = [\"b\", \"a\"]  # noqa: ECHO004\n");

    assert!(diags.is_empty());
}

#[test]
fn noqa_suppresses_mixed() {
    let diags = lint("x = [\"a\", 1]  # noqa: ECHO002\n");

    assert!(diags.is_empty());
}

#[test]
fn type_annotation_tuple_is_skipped() {
    let diags = lint("def f() -> dict[str, Any]:\n    pass\n");

    assert!(diags.is_empty());
}

#[test]
fn ann_assign_annotation_is_skipped() {
    let diags = lint("x: dict[str, Any]\n");

    assert!(diags.is_empty());
}

#[test]
fn parametrize_rows_are_skipped() {
    let source =
        "@pytest.mark.parametrize(\"a,b\", [(2, 1), (\"z\", 0)])\ndef f(a, b):\n    pass\n";

    let diags = lint(source);

    assert!(diags.is_empty());
}

#[test]
fn pytest_param_args_are_skipped() {
    let diags = lint("pytest.param(\"b\", \"a\")\n");

    assert!(diags.is_empty());
}

#[test]
fn bare_unsorted_tuple_is_still_reported() {
    let diags = lint("x = (2, 1)\n");

    assert_eq!(RULE_NUMBERS_LIST_SORTED, diags[0].code);
}
