use crate::{ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Count]
    ///
    /// count of vec or map.
    /// - no parameter allowed.
    pub fn resolve_count_of_vec_or_map(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || {
            let count = match context.deref() {
                ArcTopicDataValue::Vec(vec) => BigDecimal::from_usize(vec.len()),
                ArcTopicDataValue::Map(map) => BigDecimal::from_usize(map.len()),
                other => return self.func_not_supported(other),
            };

            self.value_as_num(count)
        })
    }
}
