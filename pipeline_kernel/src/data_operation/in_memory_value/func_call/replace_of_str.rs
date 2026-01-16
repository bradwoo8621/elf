use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Replace]
    ///
    /// replace the specified string in the given string with the given new string.
    /// - two and only two parameters are accepted,
    /// - parameters must be string
    pub fn resolve_replace_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::Str(str) => match params.len() {
                0 | 1 => self.param_count_not_enough(self.func(), 0),
                2 => {
                    let old_sub = match params[0].deref() {
                        ArcTopicDataValue::Str(sub) => sub.deref(),
                        other => return self.param_must_be_str(self.func(), 0, other),
                    };
                    if old_sub.len() == 0 {
                        return Ok(context);
                    }
                    let new_sub = match params[1].deref() {
                        ArcTopicDataValue::Str(sub) => sub.deref(),
                        other => return self.param_must_be_str(self.func(), 1, other),
                    };
                    Ok(ArcTopicDataValue::arc_from(
                        str.replace(old_sub, new_sub).to_string(),
                    ))
                }
                cnt => self.param_count_too_many(self.func(), cnt),
            },
            other => self.func_not_supported(other),
        }
    }
}
