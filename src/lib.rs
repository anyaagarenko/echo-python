mod check;
mod checker;
mod common;
mod diagnostic;
mod locator;
mod noqa;
mod rules;

pub use check::{CheckResult, check_path, check_paths, check_source};
pub use diagnostic::Diagnostic;

pub const RULE_MIXED_LIST_SORTED: &str = "echo-mixed-list-sorted";
pub const RULE_NUMBERS_LIST_SORTED: &str = "echo-numbers-list-sorted";
pub const RULE_WORDS_LIST_SORTED: &str = "echo-words-list-sorted";
