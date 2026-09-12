use crate::common::sort_key::SortKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Kind {
    Mixed,
    Numbers,
    Words,
}

pub(super) fn classify(keys: &[SortKey]) -> Option<Kind> {
    if keys.iter().all(SortKey::is_number) {
        return Some(Kind::Numbers);
    }
    if keys.iter().all(SortKey::is_word) {
        return Some(Kind::Words);
    }
    is_mixed(keys).then_some(Kind::Mixed)
}

fn is_mixed(keys: &[SortKey]) -> bool {
    let has_number = keys.iter().any(SortKey::is_number);
    let has_word = keys.iter().any(SortKey::is_word);
    has_number && has_word && keys.iter().all(|key| key.is_number() || key.is_word())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::ast::bigint::BigInt;

    #[test]
    fn classifies_numbers() {
        let keys = [SortKey::Int(BigInt::from(1)), SortKey::Int(BigInt::from(2))];

        assert_eq!(Some(Kind::Numbers), classify(&keys));
    }

    #[test]
    fn classifies_words() {
        let keys = [SortKey::Word("a".into()), SortKey::Word("b".into())];

        assert_eq!(Some(Kind::Words), classify(&keys));
    }

    #[test]
    fn classifies_mixed() {
        let keys = [SortKey::Int(BigInt::from(1)), SortKey::Word("a".into())];

        assert_eq!(Some(Kind::Mixed), classify(&keys));
    }
}
