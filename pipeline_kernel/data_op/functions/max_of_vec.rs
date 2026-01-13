use crate::{ArcTopicDataValue, Minmax};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// refer to [min_of_vec]
    pub fn max<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_value(not_support),
            _ => Err(not_support()),
        }
    }
}
