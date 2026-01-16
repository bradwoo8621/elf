use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::ConcatWith]
    ///
    /// concat strings with separator, context and params must, can be cast to string.
    /// - the first parameter is separator,
    /// - the rest parameters are parts, even part is none or empty string, still count in.
    pub fn resolve_concat_with_of_non_vec_or_map(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match params.len() {
            0 | 1 => self.param_count_not_enough(self.func(), 0),
            _ => {
                let mut result = self.unwrap_as_str(&context)?;
                let separator = self.unwrap_as_str(&params[0])?;
                for param in params.iter().skip(1) {
                    result.push_str(&separator);
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
