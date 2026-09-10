mod check;
mod checker;
mod common;
mod diagnostic;
mod locator;
mod noqa;
mod rules;
mod settings;

pub use check::{CheckResult, check_path, check_paths, check_source};
pub use diagnostic::Diagnostic;
pub use settings::CheckOptions;

pub const RULE_CALLS_USE_KWARGS: &str = "ECHO001";
pub const RULE_MIXED_LIST_SORTED: &str = "ECHO002";
pub const RULE_NUMBERS_LIST_SORTED: &str = "ECHO003";
pub const RULE_WORDS_LIST_SORTED: &str = "ECHO004";
pub const RULE_PARAMS_ONE_PER_LINE: &str = "ECHO005";
