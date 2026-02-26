use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::{StdR, StringConverterFrom};
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Join]
    ///
    /// for vec, join elements as string;
    /// for other non-map values, return to string value.
    /// - context can be anything but map,
    /// - when zero parameter, use empty string as joiner.
    ///   or use comma as joiner when [FUNC_JOIN_DEFAULT_USE_COMMA] is enabled (default disabled),
    /// - when one parameter, must be string, as joiner,
    /// - the rest parameters are parts, even part is none or empty string, still count in.
    /// - element of vec cannot be vec or map.
    ///
    /// TIP differences with the python version as below,
    ///
    /// | feature | python | rust | description |
    /// | ------- | ------ | ---- | ----------- |
    /// | zero parameter use comma | default enabled | default disabled, enabled by env variable `FUNC_JOIN_DEFAULT_USE_COMMA` | |
    /// | none value to `None` | yes | no | it is incorrect, none value should be treated as empty string |
    pub fn resolve_join_of_non_map(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let has_separator_param = self.zero_or_one_param(
            &params, //
            || Ok(false),
            |_| Ok(true),
        )?;

        match context.deref() {
            ArcTopicDataValue::None => Ok(ArcTopicDataValue::arc_from("".to_string())),
            ArcTopicDataValue::Str(s) => Ok(Arc::new(ArcTopicDataValue::Str(s.clone()))),
            ArcTopicDataValue::Num(n) => Ok(ArcTopicDataValue::arc_from(String::from_decimal(n))),
            ArcTopicDataValue::Bool(b) => Ok(ArcTopicDataValue::arc_from(String::from_bool(b))),
            ArcTopicDataValue::DateTime(dt) => {
                Ok(ArcTopicDataValue::arc_from(String::from_datetime(dt)))
            }
            ArcTopicDataValue::Date(d) => Ok(ArcTopicDataValue::arc_from(String::from_date(d))),
            ArcTopicDataValue::Time(t) => Ok(ArcTopicDataValue::arc_from(String::from_time(t))),
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    Ok(ArcTopicDataValue::arc_from("".to_string()))
                } else {
                    let separator = if has_separator_param {
                        if self.param_is_none(&params[0]) {
                            if Self::is_func_join_default_use_comma() {
                                &",".to_string()
                            } else {
                                &"".to_string()
                            }
                        } else {
                            self.param_to_str(&params[0], 0)?
                        }
                    } else if Self::is_func_join_default_use_comma() {
                        &",".to_string()
                    } else {
                        &"".to_string()
                    };
                    let mut str_vec = vec![];
                    for elm in vec.iter() {
                        str_vec.push(self.unwrap_as_str(elm)?);
                    }
                    Ok(ArcTopicDataValue::arc_from(str_vec.join(separator)))
                }
            }
            ArcTopicDataValue::Map(_) => self.func_not_supported(context.deref())?,
        }
    }
}
