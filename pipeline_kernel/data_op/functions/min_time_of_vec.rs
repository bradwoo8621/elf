use std::sync::Arc;
use elf_base::{StdErr, StdR};
use crate::{ArcTopicDataValue, Minmax};

impl ArcTopicDataValue {
	/// refer to [min_of_vec], but only time
	pub fn min_time<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
	where
		NotSupport: Fn() -> StdErr,
	{
		match self {
			ArcTopicDataValue::Vec(vec) => vec.min_time_value(not_support),
			_ => Err(not_support()),
		}
	}
}