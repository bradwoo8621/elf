use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, Zero};
use elf_base::{NumericUtils, StdR};
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Sum]
    ///
    /// compute sum value of given vec. none or empty string element treated as 0.
    /// - no parameter allowed.
    pub fn resolve_sum_of_vec(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || {
            match context.deref() {
                ArcTopicDataValue::Vec(vec) => {
                    let mut sum: BigDecimal = BigDecimal::zero();
                    for value in vec.iter() {
                        match value.deref() {
                            ArcTopicDataValue::None => continue,
                            ArcTopicDataValue::Str(str) => {
                                if !str.is_empty() {
                                    if let Ok(decimal) = str.to_decimal() {
                                        sum = sum + decimal;
                                    } else {
                                        return self.func_not_supported(context.deref());
                                    }
                                } else {
                                    // empty treated as 0
                                    continue;
                                }
                            }
                            ArcTopicDataValue::Num(num) => {
                                sum = sum + num.deref().clone();
                            }
                            _ => return self.func_not_supported(context.deref()),
                        }
                    }
                    Ok(ArcTopicDataValue::arc_from(sum))
                }
                other => self.func_not_supported(other),
            }
        })
    }
}
