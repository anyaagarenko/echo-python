use super::SortKey;

impl SortKey {
    pub(crate) const fn is_number(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    pub(crate) const fn is_word(&self) -> bool {
        matches!(self, Self::Word(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::ast::bigint::BigInt;

    #[test]
    fn int_is_number() {
        assert!(SortKey::Int(BigInt::from(1)).is_number());
    }

    #[test]
    fn float_is_number() {
        assert!(SortKey::Float(1.5).is_number());
    }

    #[test]
    fn word_is_not_number() {
        assert!(!SortKey::Word("a".into()).is_number());
    }

    #[test]
    fn word_is_word() {
        assert!(SortKey::Word("a".into()).is_word());
    }
}
