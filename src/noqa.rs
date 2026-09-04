use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct NoqaIndex {
    lines: HashMap<usize, HashSet<String>>,
}

impl NoqaIndex {
    pub fn from_source(source: &str) -> Self {
        let mut lines = HashMap::new();
        for (idx, line) in source.lines().enumerate() {
            if let Some(codes) = parse_noqa(line) {
                lines.insert(idx + 1, codes);
            }
        }
        Self { lines }
    }

    pub fn suppresses(&self, line: usize, code: &str) -> bool {
        self.lines
            .get(&line)
            .is_some_and(|codes| codes.is_empty() || codes.contains(code))
    }
}

fn parse_noqa(line: &str) -> Option<HashSet<String>> {
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
        .map(str::to_ascii_lowercase)
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
