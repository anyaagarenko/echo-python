mod ctors;
mod methods;
mod parse;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub(crate) struct NoqaIndex {
    lines: HashMap<usize, HashSet<String>>,
}
