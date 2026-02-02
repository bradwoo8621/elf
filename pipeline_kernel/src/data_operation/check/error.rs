use crate::{ArcTopicDataValue, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR, StringConverter};

impl ArcTopicDataValue {
    pub fn display_in_error(value: &ArcTopicDataValue) -> String {
        match value {
            Self::None => String::from("none"),
            Self::Str(v) => v.to_string(),
            Self::Num(v) => String::from_decimal(v),
            Self::Bool(v) => String::from_bool(v),
            Self::Date(v) => v.to_string(),
            Self::Time(v) => v.to_string(),
            Self::DateTime(v) => v.to_string(),
            Self::Map(_) => String::from("map"),
            Self::Vec(_) => String::from("vec"),
        }
    }

    #[track_caller]
    pub fn must_compare_between_num_or_datetime<R>(&self, another: &ArcTopicDataValue) -> StdR<R> {
        PipelineKernelErrorCode::ValuesNotComparable.msg(
            format!("Comparison of [none|str|decimal|date|time|datetime] are supported, current are [one={:?}, another={:?}].",
                    Self::display_in_error(self), Self::display_in_error(another)), )
    }
}
