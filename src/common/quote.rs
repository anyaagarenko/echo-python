pub(crate) fn tick(source: &str) -> String {
    format!("`{}`", collapse(source))
}

fn collapse(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_in_backticks() {
        assert_eq!("`replace`", tick("replace"));
    }

    #[test]
    fn collapses_whitespace() {
        assert_eq!("`[2, 1]`", tick("[2,\n 1]"));
    }
}
