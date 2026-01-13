use crate::{ArcTopicDataValue, Minmax};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// refer to [max], but only datetime and date
    pub fn max_datetime_of_vec<NotSupport>(
        &self,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_datetime_value(not_support),
            _ => Err(not_support()),
        }
    }
}
