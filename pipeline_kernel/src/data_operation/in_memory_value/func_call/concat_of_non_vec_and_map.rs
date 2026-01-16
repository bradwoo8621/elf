use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Concat]
    ///
    /// concat strings, context and params must, can be cast to string
    /// none treated as empty string
    pub fn resolve_concat_of_non_vec_or_map(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match params.len() {
            0 => self.param_count_not_enough(self.func(), 0),
            _ => {
                let mut result = self.unwrap_as_str(&context)?;
                for param in params.iter() {
                    let str = self.unwrap_as_str(param.deref())?;
                    if str.len() == 0 {
                        continue;
                    } else {
                        result.push_str(&str);
                    }
                }

                Ok(ArcTopicDataValue::arc_from(result))
            }
        }
    }
}
