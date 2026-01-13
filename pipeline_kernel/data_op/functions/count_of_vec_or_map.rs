use crate::ArcTopicDataValue;
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::StdR;
use std::sync::Arc;

/// count of vec or map
impl ArcTopicDataValue {
    /// try to count, can only apply to vec or map
    /// otherwise raise error by given functions
    pub fn count_of_vec_or_map<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        let count = match self {
            Self::Vec(vec) => BigDecimal::from_usize(vec.len()),
            Self::Map(map) => BigDecimal::from_usize(map.len()),
            _ => return not_support(),
        };

        Self::value_as_num(count, decimal_parse_err)
    }
}
