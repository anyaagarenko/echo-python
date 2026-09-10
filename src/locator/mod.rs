mod ctors;
mod methods;

use rustpython_parser::text_size::TextSize;

pub(crate) struct Locator {
    line_starts: Vec<TextSize>,
}
