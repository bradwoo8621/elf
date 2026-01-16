use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, Zero};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Find], [VariablePredefineFunctions::Index]
    ///
    /// find substring start index of given string
    /// - one and only one parameter accepted,
    /// - parameter must be string
    /// - parameter string is empty, return 0
    /// - parameter string is not found, return -1
    /// - return the start index of the parameter string in given string
    // noinspection DuplicatedCode
    pub fn resolve_find_of_str(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.only_param(&params, |param| match context.deref() {
            ArcTopicDataValue::Str(str) => {
                let sub = self.param_to_str(param, 0)?;
                if sub.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(BigDecimal::zero()))
                } else if str.len() == 0 {
                    Ok(ArcTopicDataValue::arc_from(BigDecimal::from(-1)))
                } else if let Some(index) = str.find(sub) {
                    Ok(ArcTopicDataValue::arc_from(BigDecimal::from(index as u128)))
                } else {
                    Ok(ArcTopicDataValue::arc_from(BigDecimal::from(-1)))
                }
            }
            other => self.func_not_supported(other),
        })
    }
}
