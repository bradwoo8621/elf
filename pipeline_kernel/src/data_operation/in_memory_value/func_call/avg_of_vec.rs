use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, Zero};
use elf_base::{NumericUtils, StdR};
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Avg]
    ///
    /// compute avg value of given vec.
    /// - no parameter allowed.
    /// - only decimal and string which can be cast to decimal are counted in.
    /// - if no element counted in, returns none.
    pub fn resolve_avg_of_vec(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || {
            match context.deref() {
                ArcTopicDataValue::Vec(vec) => {
                    let mut count: i32 = 0;
                    let mut sum: BigDecimal = BigDecimal::zero();
                    for value in vec.iter() {
                        match value.deref() {
                            ArcTopicDataValue::None => continue,
                            ArcTopicDataValue::Str(str) => {
                                if !str.is_empty() {
                                    if let Ok(decimal) = str.to_decimal() {
                                        count += 1;
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
                                count += 1;
                                sum = sum + num.deref().clone();
                            }
                            _ => return self.func_not_supported(context.deref()),
                        }
                    }

                    if count == 0 {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    } else {
                        Ok(ArcTopicDataValue::arc_from(sum / count))
                    }
                }
                other => self.func_not_supported(other),
            }
        })
    }
}
