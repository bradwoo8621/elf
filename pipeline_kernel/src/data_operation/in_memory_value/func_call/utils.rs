use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::BigDecimal;
use elf_base::StdR;
use std::sync::Arc;

/// utilities
impl InMemoryFuncCall<'_> {
    /// convert to [ArcTopicDataValue::Num] if value is some.
    /// or raise error if value is none
    pub fn value_as_num(&self, value: Option<BigDecimal>) -> StdR<Arc<ArcTopicDataValue>> {
        value
            .map(|value| Ok(ArcTopicDataValue::arc_from(value)))
            .unwrap_or_else(|| self.decimal_parse_error("none"))
    }

    pub fn try_to_usize<CannotCast>(
        &self,
        param: &ArcTopicDataValue,
        none_value: usize,
        cannot_cast: CannotCast,
    ) -> StdR<usize>
    where
        CannotCast: FnOnce() -> StdR<usize>,
    {
        if let Ok(value) = param.try_to_usize_or_if_none(none_value) {
            Ok(value)
        } else {
            cannot_cast()
        }
    }
}
