use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Trim], [VariablePredefineFunctions::Strip]
    ///
    /// trim given string by given trimmed part
    /// - no parameter, trim whitespaces,
    /// - one parameter, must be string, trim it
    // noinspection DuplicatedCode
    pub fn resolve_trim_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(context),
                _ => match params.len() {
                    0 => Ok(ArcTopicDataValue::arc_from(str.deref().trim().to_string())),
                    1 => {
                        let matches = match params[0].deref() {
                            ArcTopicDataValue::Str(sub) => sub.deref().clone(),
                            other => return self.param_must_be_str(self.func(), 0, other),
                        };
                        Ok(ArcTopicDataValue::arc_from(
                            str.deref()
                                .trim_matches(&matches.chars().collect::<Vec<char>>()[..])
                                .to_string(),
                        ))
                    }
                    cnt => return self.param_count_too_many(self.func(), cnt),
                },
            },
            other => self.func_not_supported(other),
        }
    }
}
