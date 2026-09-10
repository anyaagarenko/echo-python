mod visit;

use crate::bindings::Bindings;
use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;
use crate::settings::Settings;

pub(crate) struct Checker<'a> {
    pub(crate) bindings: &'a Bindings,
    pub(crate) diagnostics: &'a mut Vec<Diagnostic>,
    pub(crate) locator: &'a Locator,
    pub(crate) noqa: &'a NoqaIndex,
    pub(crate) path: &'a std::path::Path,
    pub(crate) settings: &'a Settings,
}
