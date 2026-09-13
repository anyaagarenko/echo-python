mod load;
mod rules;

use std::collections::HashSet;

pub use rules::CheckOptions;

pub(crate) use load::load_for_path;

#[derive(Clone, Debug, Default)]
pub(crate) struct Settings {
    pub(crate) enabled: HashSet<String>,
    pub(crate) echo001: Echo001Settings,
    pub(crate) echo006: Echo006Settings,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Echo001Settings {
    pub(crate) ignore: HashSet<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Echo006Settings {
    pub(crate) names: HashSet<String>,
}

impl Settings {
    pub(crate) fn is_enabled(&self, code: &str) -> bool {
        self.enabled.contains(code)
    }
}

impl Echo001Settings {
    pub(crate) fn ignores(&self, name: &str) -> bool {
        self.ignore.contains(name)
    }
}

impl Default for Echo006Settings {
    fn default() -> Self {
        Self {
            names: HashSet::from(["msg".to_string()]),
        }
    }
}

impl Echo006Settings {
    pub(crate) fn from_config(names: Vec<String>, allow_msg: bool) -> Self {
        let mut restricted: HashSet<String> = names.into_iter().collect();
        if !allow_msg {
            restricted.insert("msg".to_string());
        }
        Self { names: restricted }
    }

    pub(crate) fn is_restricted(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}
