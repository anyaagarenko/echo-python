mod load;

use std::collections::HashSet;

pub(crate) use load::load_for_path;

#[derive(Clone, Debug, Default)]
pub(crate) struct Settings {
    pub(crate) calls_use_kwargs: CallsUseKwargsSettings,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CallsUseKwargsSettings {
    pub(crate) ignore: HashSet<String>,
}

impl CallsUseKwargsSettings {
    pub(crate) fn ignores(&self, name: &str) -> bool {
        self.ignore.contains(name)
    }
}
