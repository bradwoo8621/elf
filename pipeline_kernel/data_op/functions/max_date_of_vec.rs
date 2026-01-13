use crate::{ArcTopicDataValue, Minmax};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// refer to [max_of_vec], but only date
    pub fn max_date_of_vec<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_date_value(not_support),
            _ => Err(not_support()),
        }
    }
}
