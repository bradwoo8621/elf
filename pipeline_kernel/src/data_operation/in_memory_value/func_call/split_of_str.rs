use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Split]
    ///
    /// split given string by given separator
    /// - no parameter, use default separator [,],
    /// - one parameter, must be string, split by it
    pub fn resolve_split_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(context),
                _ => match params.len() {
                    0 => Ok(ArcTopicDataValue::arc_from(
                        str.split(',')
                            .map(|s| ArcTopicDataValue::arc_from(s.to_string()))
                            .collect::<Vec<Arc<ArcTopicDataValue>>>(),
                    )),
                    1 => {
                        let separator = self.param_to_str(&params[0], 0)?;
                        Ok(ArcTopicDataValue::arc_from(
                            str.split(separator)
                                .map(|s| ArcTopicDataValue::arc_from(s.to_string()))
                                .collect::<Vec<Arc<ArcTopicDataValue>>>(),
                        ))
                    }
                    cnt => self.param_count_too_many(self.func(), cnt),
                },
            },
            other => self.func_not_supported(other),
        }
    }
}
