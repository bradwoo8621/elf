use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Lower]
    ///
    /// lower given string
    pub fn resolve_lower_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || match context.deref() {
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(context),
                _ => Ok(ArcTopicDataValue::arc_from(str.to_lowercase().to_string())),
            },
            other => self.func_not_supported(other),
        })
    }
}
