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
        let separator = self.zero_or_one_param(
            &params,
            || Ok(",".to_string()),
            |param| {
                if self.param_is_none(param) {
                    Ok(",".to_string())
                } else {
                    Ok(self.param_to_str(param, 0)?.to_string())
                }
            },
        )?;
        match context.deref() {
            ArcTopicDataValue::Str(str) => match str.len() {
                0 => Ok(ArcTopicDataValue::arc_from(vec![
                    ArcTopicDataValue::arc_from(String::new()),
                ])),
                _ => Ok(ArcTopicDataValue::arc_from(
                    str.split(&separator)
                        .map(|s| ArcTopicDataValue::arc_from(s.to_string()))
                        .collect::<Vec<Arc<ArcTopicDataValue>>>(),
                )),
            },
            other => self.func_not_supported(other),
        }
    }
}
