mod load;
mod rules;

use std::collections::HashSet;

pub use rules::CheckOptions;

pub(crate) use load::load_for_path;

#[derive(Clone, Debug, Default)]
pub(crate) struct Settings {
    pub(crate) enabled: HashSet<String>,
    pub(crate) echo001: Echo001Settings,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Echo001Settings {
    pub(crate) ignore: HashSet<String>,
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
