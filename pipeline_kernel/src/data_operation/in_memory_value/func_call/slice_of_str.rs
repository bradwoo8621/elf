use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

/// [VariablePredefineFunctions::Slice], [VariablePredefineFunctions::Substr]
impl InMemoryFuncCall<'_> {
    /// get slice of str.
    /// from start index (included, default 0), to end index (excluded, default chars count of given string)
    /// - one or two parameters accepted,
    /// - parameters must be none or can be cast to usize,
    /// - use 0 if the first parameter is none,
    /// - use chars count if then second parameter is none,
    /// - return the slice string of given string
    // noinspection DuplicatedCode
    pub fn resolve_slice_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match context.deref() {
            ArcTopicDataValue::None => Ok(Arc::new(ArcTopicDataValue::None)),
            ArcTopicDataValue::Str(str) => {
                if str.is_empty() {
                    Ok(Arc::new(ArcTopicDataValue::Str(str.clone())))
                } else {
                    let (start_index, end_index) = match params.len() {
                        0 => return self.param_count_not_enough(self.func(), 0),
                        1 => {
                            let start_index = self.param_to_usize(&params[0], 0, 0)?;
                            let end_index = str.chars().count();
                            (start_index, end_index)
                        }
                        2 => {
                            let start_index = self.param_to_usize(&params[0], 0, 0)?;
                            let end_index =
                                self.param_to_usize(&params[1], str.chars().count(), 1)?;
                            (start_index, end_index)
                        }
                        cnt => return self.param_count_too_many(self.func(), cnt),
                    };
                    let sliced = str
                        .chars()
                        .skip(start_index)
                        .take(end_index - start_index)
                        .collect::<String>();
                    Ok(ArcTopicDataValue::arc_from(sliced))
                }
            }
            other => self.func_not_supported(other),
        }
    }
}
