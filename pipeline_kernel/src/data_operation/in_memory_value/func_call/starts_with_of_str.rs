use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use elf_model::VariablePredefineFunctions;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::StartsWith], [VariablePredefineFunctions::Startswith]
    /// 
    /// check given string starts with substring or not
    /// - one and only one parameter accepted,
    /// - parameter must be string
    /// - parameter string is empty, return true
    /// - return true when given string starts with parameter string
    // noinspection DuplicatedCode
    pub fn resolve_starts_with_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::Str(str) => {
                let sub = match params.len() {
                    0 => return self.param_count_not_enough(self.func(), 0),
                    1 => match params[0].deref() {
                        ArcTopicDataValue::Str(sub) => sub.deref(),
                        other => return self.param_must_be_str(self.func(), 0, other),
                    },
                    cnt => return self.param_count_too_many(self.func(), cnt),
                };
                if sub.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(true))
                } else if str.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(false))
                } else {
                    Ok(ArcTopicDataValue::arc_from(str.starts_with(sub)))
                }
            }
            other => self.func_not_supported(other),
        }
    }
}
