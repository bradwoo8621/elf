use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Upper]
    ///
    /// upper given string
    pub fn resolve_upper_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || match context.deref() {
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(context),
                _ => Ok(ArcTopicDataValue::arc_from(str.to_uppercase().to_string())),
            },
            other => self.func_not_supported(other),
        })
    }
}
