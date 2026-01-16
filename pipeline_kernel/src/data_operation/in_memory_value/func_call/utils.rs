use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::BigDecimal;
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

/// utilities
impl InMemoryFuncCall<'_> {
    /// convert to [ArcTopicDataValue::Num] if value is some.
    /// or raise error if value is none
    pub fn value_as_num(&self, value: Option<BigDecimal>) -> StdR<Arc<ArcTopicDataValue>> {
        value
            .map(|value| Ok(ArcTopicDataValue::arc_from(value)))
            .unwrap_or_else(|| self.decimal_parse_error("none"))
    }

    pub fn try_to_usize<CannotCast>(
        &self,
        param: &ArcTopicDataValue,
        none_value: usize,
        cannot_cast: CannotCast,
    ) -> StdR<usize>
    where
        CannotCast: FnOnce() -> StdR<usize>,
    {
        if let Ok(value) = param.try_to_usize_or_if_none(none_value) {
            Ok(value)
        } else {
            cannot_cast()
        }
    }

    pub fn no_param<R, DoWhenNoParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_no_param: DoWhenNoParam,
    ) -> StdR<R>
    where
        DoWhenNoParam: FnOnce() -> StdR<R>,
    {
        let count = params.len();
        if count > 0 {
            self.param_count_too_many(self.func(), count)
        } else {
            do_when_no_param()
        }
    }

    pub fn only_param<R, DoWhenOnlyParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_only_param: DoWhenOnlyParam,
    ) -> StdR<R>
    where
        DoWhenOnlyParam: FnOnce(&ArcTopicDataValue) -> StdR<R>,
    {
        match params.len() {
            0 => self.param_count_not_enough(self.func(), 0),
            1 => do_when_only_param(params[0].deref()),
            cnt => self.param_count_too_many(self.func(), cnt),
        }
    }
}
