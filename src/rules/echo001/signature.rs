use crate::bindings::FunctionShape;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Slot {
    Normal,
    PosOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Signature {
    slots: Vec<Slot>,
    var_positional: bool,
}

impl Signature {
    pub(super) fn from_function_shape(shape: FunctionShape) -> Self {
        let mut slots = Vec::with_capacity(shape.posonly + shape.args);
        slots.extend(std::iter::repeat_n(Slot::PosOnly, shape.posonly));
        slots.extend(std::iter::repeat_n(Slot::Normal, shape.args));
        Self {
            slots,
            var_positional: shape.vararg,
        }
    }

    pub(super) fn positional_only(count: usize) -> Self {
        Self {
            slots: vec![Slot::PosOnly; count],
            var_positional: false,
        }
    }

    pub(super) const fn varargs() -> Self {
        Self {
            slots: Vec::new(),
            var_positional: true,
        }
    }

    pub(super) fn binds_keyword_capable(&self, positional_count: usize) -> bool {
        let mut remaining = positional_count;
        for slot in &self.slots {
            if remaining == 0 {
                return false;
            }
            remaining -= 1;
            if matches!(slot, Slot::Normal) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_only_does_not_bind_keywords() {
        assert!(!Signature::positional_only(2).binds_keyword_capable(2));
    }

    #[test]
    fn normal_binds_keywords() {
        let signature = Signature {
            slots: vec![Slot::Normal, Slot::Normal],
            var_positional: false,
        };
        assert!(signature.binds_keyword_capable(2));
    }

    #[test]
    fn varargs_do_not_bind_keywords() {
        assert!(!Signature::varargs().binds_keyword_capable(2));
    }

    #[test]
    fn mixed_second_slot_can_bind() {
        let signature = Signature {
            slots: vec![Slot::PosOnly, Slot::Normal],
            var_positional: false,
        };
        assert!(signature.binds_keyword_capable(2));
        assert!(!signature.binds_keyword_capable(1));
    }
}
