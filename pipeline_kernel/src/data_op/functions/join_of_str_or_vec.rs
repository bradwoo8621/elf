use crate::ArcTopicDataValue;
use elf_base::{StdR, StringConverter};
use std::ops::Deref;
use std::sync::Arc;

impl ArcTopicDataValue {
    /// 1. return cloned string when self is string
    /// 2. return joined string when self is vec, and element of vec cannot be vec or map. note the none value is ignored
    pub fn join_of_str_or_vec<NotSupport>(
        &self,
        sep: &str,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Str(str) => Ok(Arc::new(Self::Str(str.clone()))),
            Self::Vec(vec) => {
                if vec.len() == 0 {
                    Ok(Arc::new(Self::Str(Arc::new("".to_string()))))
                } else {
                    let mut segments: Vec<String> = vec![];
                    for value in vec.iter() {
                        match value.deref() {
                            Self::Str(str) => {
                                segments.push(str.to_string());
                            }
                            Self::Num(decimal) => {
                                segments.push(String::from_decimal(decimal.deref()));
                            }
                            Self::Bool(bool) => {
                                segments.push(String::from_bool(bool));
                            }
                            Self::DateTime(datetime) => {
                                segments.push(String::from_datetime(datetime));
                            }
                            Self::Date(date) => {
                                segments.push(String::from_date(date));
                            }
                            Self::Time(time) => {
                                segments.push(String::from_time(time));
                            }
                            Self::None => {}
                            _ => return not_support(),
                        }
                    }
                    Ok(Arc::new(Self::Str(Arc::new(segments.join(sep)))))
                }
            }
            _ => not_support(),
        }
    }
}
