use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::EndsWith], [VariablePredefineFunctions::Endswith]
    ///
    /// check given string ends with substring or not
    /// - one and only one parameter accepted,
    /// - parameter must be string
    /// - parameter string is empty, return true
    /// - return true when given string ends with parameter string
    // noinspection DuplicatedCode
    pub fn resolve_ends_with_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.only_param(&params, |param| match context.deref() {
            ArcTopicDataValue::Str(str) => {
                let sub = match param {
                    ArcTopicDataValue::Str(sub) => sub.deref(),
                    other => return self.param_must_be_str(self.func(), 0, other),
                };
                if sub.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(true))
                } else if str.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(false))
                } else {
                    Ok(ArcTopicDataValue::arc_from(str.ends_with(sub)))
                }
            }
            other => self.func_not_supported(other),
        })
    }
}
