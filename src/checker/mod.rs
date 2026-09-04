mod visit;

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;
use crate::noqa::NoqaIndex;

pub(crate) struct Checker<'a> {
    pub(crate) locator: &'a Locator,
    pub(crate) noqa: &'a NoqaIndex,
    pub(crate) path: &'a std::path::Path,
    pub(crate) diagnostics: &'a mut Vec<Diagnostic>,
}
