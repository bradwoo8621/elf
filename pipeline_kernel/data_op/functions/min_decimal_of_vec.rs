use crate::{ArcTopicDataValue, Minmax};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// refer to [min_of_vec], but only decimal and string
    pub fn min_decimal_of_vec<NotSupport>(
        &self,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_decimal_value(not_support),
            _ => Err(not_support()),
        }
    }
}
