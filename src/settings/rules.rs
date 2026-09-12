use std::collections::HashSet;

use crate::{
    RULE_ECHO001, RULE_MIXED_LIST_SORTED, RULE_NUMBERS_LIST_SORTED, RULE_PARAMS_ONE_PER_LINE,
    RULE_WORDS_LIST_SORTED,
};

pub(crate) const ALL_RULES: &[&str] = &[
    RULE_ECHO001,
    RULE_MIXED_LIST_SORTED,
    RULE_NUMBERS_LIST_SORTED,
    RULE_WORDS_LIST_SORTED,
    RULE_PARAMS_ONE_PER_LINE,
];

#[derive(Clone, Debug, Default)]
pub struct CheckOptions {
    pub select: Option<Vec<String>>,
    pub extend_select: Vec<String>,
    pub ignore: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct LintFileSettings {
    pub(crate) select: Option<Vec<String>>,
    pub(crate) extend_select: Vec<String>,
    pub(crate) ignore: Vec<String>,
}

pub(crate) fn resolve_enabled(file: &LintFileSettings, cli: &CheckOptions) -> HashSet<String> {
    let select = cli
        .select
        .clone()
        .or_else(|| file.select.clone())
        .unwrap_or_else(|| vec!["ALL".to_string()]);
    let mut enabled = expand_selectors(&select);
    for selector in file.extend_select.iter().chain(cli.extend_select.iter()) {
        enabled.extend(expand_selector(selector));
    }
    for selector in file.ignore.iter().chain(cli.ignore.iter()) {
        remove_selector(&mut enabled, selector);
    }
    enabled
}

fn expand_selectors(selectors: &[String]) -> HashSet<String> {
    let mut enabled = HashSet::new();
    for selector in selectors {
        enabled.extend(expand_selector(selector));
    }
    enabled
}

fn expand_selector(selector: &str) -> HashSet<String> {
    if selector == "ALL" {
        return ALL_RULES.iter().map(|rule| (*rule).to_string()).collect();
    }
    ALL_RULES
        .iter()
        .filter(|rule| selector_matches(selector, rule))
        .map(|rule| (*rule).to_string())
        .collect()
}

fn remove_selector(enabled: &mut HashSet<String>, selector: &str) {
    if selector == "ALL" {
        enabled.clear();
        return;
    }
    enabled.retain(|rule| !selector_matches(selector, rule));
}

fn selector_matches(selector: &str, rule: &str) -> bool {
    let selector = selector.to_ascii_uppercase();
    let rule = rule.to_ascii_uppercase();
    rule == selector || rule.starts_with(&selector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_selects_every_rule() {
        let enabled = expand_selector("ALL");

        assert_eq!(ALL_RULES.len(), enabled.len());
    }

    #[test]
    fn prefix_selects_group() {
        let enabled = expand_selector("ECHO003");

        assert!(enabled.contains(RULE_NUMBERS_LIST_SORTED));
        assert!(!enabled.contains(RULE_WORDS_LIST_SORTED));
    }

    #[test]
    fn echo_prefix_selects_all_echo_rules() {
        let enabled = expand_selector("ECHO");

        assert_eq!(ALL_RULES.len(), enabled.len());
    }

    #[test]
    fn cli_select_replaces_file_select() {
        let file = LintFileSettings {
            select: Some(vec!["ECHO003".into()]),
            ..LintFileSettings::default()
        };
        let cli = CheckOptions {
            select: Some(vec!["ECHO004".into()]),
            ..CheckOptions::default()
        };

        let enabled = resolve_enabled(&file, &cli);

        assert!(enabled.contains(RULE_WORDS_LIST_SORTED));
        assert!(!enabled.contains(RULE_NUMBERS_LIST_SORTED));
    }

    #[test]
    fn ignore_removes_selected() {
        let file = LintFileSettings {
            select: Some(vec!["ALL".into()]),
            ignore: vec!["ECHO001".into()],
            ..LintFileSettings::default()
        };

        let enabled = resolve_enabled(&file, &CheckOptions::default());

        assert!(!enabled.contains(RULE_ECHO001));
        assert!(enabled.contains(RULE_NUMBERS_LIST_SORTED));
    }

    #[test]
    fn extend_select_adds() {
        let file = LintFileSettings {
            select: Some(vec!["ECHO003".into()]),
            extend_select: vec!["ECHO004".into()],
            ..LintFileSettings::default()
        };

        let enabled = resolve_enabled(&file, &CheckOptions::default());

        assert!(enabled.contains(RULE_NUMBERS_LIST_SORTED));
        assert!(enabled.contains(RULE_WORDS_LIST_SORTED));
    }
}
