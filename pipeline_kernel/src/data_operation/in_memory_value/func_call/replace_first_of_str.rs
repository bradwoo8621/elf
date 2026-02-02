use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::ReplaceFirst]
    ///
    /// replace the specified string in the given string (none treated as empty string) with the given new string,
    /// only the first one is replaced.
    /// - two and only two parameters are accepted,
    /// - parameters must be string
    pub fn resolve_replace_first_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.two_params(&params, |first_param, second_param| match context.deref() {
            ArcTopicDataValue::None => Ok(ArcTopicDataValue::arc_from(String::new())),
            ArcTopicDataValue::Str(str) => {
                let old_sub = if self.param_is_none(first_param) {
                    &String::new()
                } else {
                    self.param_to_str(first_param, 0)?
                };
                let new_sub = if self.param_is_none(second_param) {
                    &String::new()
                } else {
                    self.param_to_str(second_param, 1)?
                };
                if old_sub == new_sub {
                    Ok(context)
                } else {
                    Ok(ArcTopicDataValue::arc_from(
                        str.replacen(old_sub, new_sub, 1).to_string(),
                    ))
                }
            }
            other => self.func_not_supported(other),
        })
    }
}
