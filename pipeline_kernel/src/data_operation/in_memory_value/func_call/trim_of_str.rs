use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Trim], [VariablePredefineFunctions::Strip]
    ///
    /// trim given string (none treated as empty string) by given trimmed part
    /// - no parameter, trim whitespaces,
    /// - one parameter, must be string, trim it
    pub fn resolve_trim_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::None => Ok(ArcTopicDataValue::arc_from(String::new())),
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(context),
                _ => match params.len() {
                    0 => Ok(ArcTopicDataValue::arc_from(str.trim().to_string())),
                    1 => {
                        if self.param_is_none(&params[0]) {
                            Ok(ArcTopicDataValue::arc_from(str.trim().to_string()))
                        } else {
                            let matches = self.param_to_str(&params[0], 0)?;
                            Ok(ArcTopicDataValue::arc_from(
                                str.trim_matches(&matches.chars().collect::<Vec<char>>()[..])
                                    .to_string(),
                            ))
                        }
                    }
                    cnt => self.param_count_too_many(self.func(), cnt),
                },
            },
            other => self.func_not_supported(other),
        }
    }
}
