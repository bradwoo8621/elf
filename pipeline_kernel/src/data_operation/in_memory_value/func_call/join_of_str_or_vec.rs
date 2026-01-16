use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Join]
    ///
    /// concat strings with separator, context and params must, can be cast to string.
    /// - zero parameter use comma as separator (env variable enabled), otherwise default use empty string.
    /// - the rest parameters are parts, even part is none or empty string, still count in.
    /// - return self when self is string
    /// - return joined string when self is vec, and element of vec cannot be vec or map.
    ///
    /// TIP differences with the python version as below,
    ///
    /// | feature | python | rust | description |
    /// | ------- | ------ | ---- | ----------- |
    /// | zero parameter use comma | default enabled | default disabled, enabled by env variable `FUNC_JOIN_DEFAULT_USE_COMMA` | |
    /// | none value to `None` | yes | no | it is incorrect, none value should be treated as empty string |
    ///
    pub fn resolve_join_of_str_or_vec(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let has_separator_param = self.zero_or_one_param(
            &params, //
            || Ok(false),
            |_| Ok(true),
        )?;
        let separator = if has_separator_param {
            self.param_to_str(&params[0], 0)?
        } else if Self::is_func_join_default_use_comma() {
            &",".to_string()
        } else {
            &"".to_string()
        };

        match context.deref() {
            ArcTopicDataValue::Str(s) => Ok(Arc::new(ArcTopicDataValue::Str(s.clone()))),
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    Ok(ArcTopicDataValue::arc_from("".to_string()))
                } else {
                    let mut str_vec = vec![];
                    for elm in vec.iter() {
                        str_vec.push(self.unwrap_as_str(elm)?);
                    }
                    Ok(ArcTopicDataValue::arc_from(str_vec.join(separator)))
                }
            }
            other => self.func_not_supported(other)?,
        }
    }
}
