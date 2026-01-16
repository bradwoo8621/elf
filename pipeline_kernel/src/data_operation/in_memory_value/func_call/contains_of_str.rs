use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Contains]
    ///
    /// check given string (none treated as empty string) contains substring or not
    /// - one and only one parameter accepted,
    /// - parameter must be string
    /// - parameter string is empty, return true
    /// - return true when given string contains parameter string
    // noinspection DuplicatedCode
    pub fn resolve_contains_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.one_param(&params, |param| match context.deref() {
            ArcTopicDataValue::None => {
                let sub = self.param_to_str(param, 0)?;
                if sub.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(true))
                } else {
                    Ok(ArcTopicDataValue::arc_from(false))
                }
            }
            ArcTopicDataValue::Str(str) => {
                let sub = self.param_to_str(param, 0)?;
                if sub.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(true))
                } else if str.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(false))
                } else {
                    Ok(ArcTopicDataValue::arc_from(str.contains(sub)))
                }
            }
            other => self.func_not_supported(other),
        })
    }
}
