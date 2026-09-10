use std::collections::HashSet;

pub(super) fn parse_noqa(line: &str) -> Option<HashSet<String>> {
    let hash = line.find('#')?;
    let comment = line[hash + 1..].trim_start();
    let rest = strip_noqa_prefix(comment)?;
    let rest = rest.trim_start();

    if rest.is_empty() || rest.starts_with('#') {
        return Some(HashSet::new());
    }

    let Some(after_colon) = rest.strip_prefix(':') else {
        return Some(HashSet::new());
    };

    let codes_part = after_colon.split('#').next().unwrap_or("").trim();
    let codes = codes_part
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_uppercase)
        .collect();

    Some(codes)
}

fn strip_noqa_prefix(comment: &str) -> Option<&str> {
    if comment.len() >= 4 && comment.as_bytes()[..4].eq_ignore_ascii_case(b"noqa") {
        Some(&comment[4..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_noqa() {
        let codes = parse_noqa("x = 1  # noqa").unwrap();
        assert!(codes.is_empty());
    }

    #[test]
    fn parses_codes_after_colon() {
        let codes = parse_noqa("x = 1  # noqa: FOO, BAR").unwrap();
        assert!(codes.contains("FOO"));
        assert!(codes.contains("BAR"));
    }

    #[test]
    fn rejects_non_noqa_comment() {
        assert!(parse_noqa("x = 1  # hello").is_none());
    }
}
