use crate::ArcTopicDataValue;
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::{StdR, StringConverter};
use std::sync::Arc;

/// length of str or num
impl ArcTopicDataValue {
    /// get chars count of string, or decimal to string
    pub fn length_of_str_or_num<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        let chars_count = match self {
            Self::Str(str) => BigDecimal::from_usize(str.chars().count()),
            Self::Num(decimal) => {
                BigDecimal::from_usize(String::from_decimal(decimal).chars().count())
            }
            _ => return not_support(),
        };

        Self::value_as_num(chars_count, decimal_parse_err)
    }
}
