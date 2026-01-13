use crate::{ArcTopicDataValue, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use std::sync::Arc;

impl ArcTopicDataValue {
    /// in when
    /// 1. another is none -> false
    /// 2. another is vec, check if there is any same,
    /// 3. another is string, split with comma, check if there is any same,
    /// 4. error.
    pub fn is_in(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match another {
			Self::None => Ok(false),
			Self::Vec(another_vec) => match self {
				Self::Vec(_) => Ok(false),
				Self::Map(_) => Ok(false),
				// same as any element in vec
				_ => Ok(another_vec.iter().any(|another_value| self.is_same_as(another_value)))
			},
			Self::Str(another_str) => match self {
				Self::Vec(_) => Ok(false),
				Self::Map(_) => Ok(false),
				_ => Ok(another_str
					.split(',')
					.into_iter()
					.map(|s| Self::Str(Arc::new(s.to_string())))
					.any(|another_value| self.is_same_as(&another_value)))
			}
			_ => PipelineKernelErrorCode::ValuesNotComparable.msg(
				format!("Comparison of [none|str|decimal|date|time|datetime] are supported, current are [one={:?}, another={:?}].",
				        Self::display_in_error(self), Self::display_in_error(another)))
		}
    }

    /// refer to [is_in].
    /// note that none is not in none.
    pub fn is_not_in(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        self.is_in(another).map(|b| !b)
    }
}
