use crate::{ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::{StdR, StringConverter};
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// length of string or decimal
    pub fn resolve_length_of_str_or_num(
        &self,
        context: Arc<ArcTopicDataValue>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let chars_count = match context.deref() {
            ArcTopicDataValue::Str(str) => BigDecimal::from_usize(str.chars().count()),
            ArcTopicDataValue::Num(decimal) => {
                BigDecimal::from_usize(String::from_decimal(decimal).chars().count())
            }
            other => return self.func_not_supported(other),
        };

        self.value_as_num(chars_count)
    }
}
