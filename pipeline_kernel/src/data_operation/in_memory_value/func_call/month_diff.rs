use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::{DateDiffUtils, StdR};
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    pub fn resolve_month_diff(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.one_param(&params, |param| {
            let start = self.unwrap_as_date(&context)?;
            let end = self.unwrap_as_date(param)?;

            Ok(ArcTopicDataValue::arc_from(
                BigDecimal::from_i64(start.month_diff(&end)).unwrap(),
            ))
        })
    }
}
