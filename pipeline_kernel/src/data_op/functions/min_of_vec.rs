use crate::{ArcTopicDataValue, Minmax};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// get the min value of vec elements, only decimal/datetime/date/time can be compared
    /// - if there is no element in vec, returns none,
    /// - none or empty string ignored,
    /// - all elements must, can be converted to one single type,
    /// - if there are datetime and date, returns date.
    pub fn min_of_vec<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_value(not_support),
            _ => Err(not_support()),
        }
    }
}
