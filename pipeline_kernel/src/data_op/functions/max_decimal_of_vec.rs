use std::sync::Arc;
use elf_base::{StdErr, StdR};
use crate::{ArcTopicDataValue, Minmax};

impl ArcTopicDataValue {
	/// refer to [max_of_vec], but only decimal and string
	pub fn max_decimal_of_vec<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
	where
		NotSupport: Fn() -> StdErr,
	{
		match self {
			ArcTopicDataValue::Vec(vec) => vec.max_decimal_value(not_support),
			_ => Err(not_support()),
		}
	}
}