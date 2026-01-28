use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use elf_base::{DateTimeUtils, StdR, StringConverter};
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

    /// convert to string if given value is not map or vec.
    /// otherwise raise error
    pub fn unwrap_as_str(&self, value: &ArcTopicDataValue) -> StdR<String> {
        let str = match value {
            ArcTopicDataValue::None => "".to_string(),
            ArcTopicDataValue::Str(s) => s.deref().clone(),
            ArcTopicDataValue::Num(n) => String::from_decimal(n),
            ArcTopicDataValue::Bool(b) => String::from_bool(b),
            ArcTopicDataValue::DateTime(dt) => String::from_datetime(dt),
            ArcTopicDataValue::Date(d) => String::from_date(d),
            ArcTopicDataValue::Time(t) => String::from_time(t),
            ArcTopicDataValue::Map(_) => self.str_parse_error(value)?,
            ArcTopicDataValue::Vec(_) => self.str_parse_error(value)?,
        };
        Ok(str)
    }

    pub fn unwrap_as_date(&self, value: &ArcTopicDataValue) -> StdR<NaiveDate> {
        let date = match value {
            ArcTopicDataValue::Str(str) => {
                if let Ok(date) = str.to_date_loose() {
                    date
                } else {
                    return self.date_parse_error(value);
                }
            }
            ArcTopicDataValue::Date(date) => date.deref().clone(),
            ArcTopicDataValue::DateTime(datetime) => datetime.date(),
            _ => return self.date_parse_error(value),
        };
        Ok(date)
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

    pub fn one_param<R, DoWhenOnlyParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_only_param: DoWhenOnlyParam,
    ) -> StdR<R>
    where
        DoWhenOnlyParam: FnOnce(&ArcTopicDataValue) -> StdR<R>,
    {
        match params.len() {
            0 => self.param_count_not_enough(self.func(), 0),
            1 => do_when_only_param(&params[0]),
            cnt => self.param_count_too_many(self.func(), cnt),
        }
    }

    pub fn two_params<R, DoWhenTwoParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_two_params: DoWhenTwoParam,
    ) -> StdR<R>
    where
        DoWhenTwoParam: FnOnce(&ArcTopicDataValue, &ArcTopicDataValue) -> StdR<R>,
    {
        match params.len() {
            0 => self.param_count_not_enough(self.func(), 0),
            1 => self.param_count_not_enough(self.func(), 1),
            2 => do_when_two_params(&params[0], &params[1]),
            cnt => self.param_count_too_many(self.func(), cnt),
        }
    }

    pub fn zero_or_one_param<R, DoWhenNoParam, DoWhenOneParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_no_param: DoWhenNoParam,
        do_when_one_param: DoWhenOneParam,
    ) -> StdR<R>
    where
        DoWhenNoParam: FnOnce() -> StdR<R>,
        DoWhenOneParam: FnOnce(&ArcTopicDataValue) -> StdR<R>,
    {
        match params.len() {
            0 => do_when_no_param(),
            1 => do_when_one_param(&params[0]),
            cnt => self.param_count_too_many(self.func(), cnt),
        }
    }

    pub fn one_or_two_params<R, DoWhenOneParam, DoWhenTwoParam>(
        &self,
        params: &Vec<Arc<ArcTopicDataValue>>,
        do_when_one_param: DoWhenOneParam,
        do_when_two_params: DoWhenTwoParam,
    ) -> StdR<R>
    where
        DoWhenOneParam: FnOnce(&ArcTopicDataValue) -> StdR<R>,
        DoWhenTwoParam: FnOnce(&ArcTopicDataValue, &ArcTopicDataValue) -> StdR<R>,
    {
        match params.len() {
            0 => self.param_count_not_enough(self.func(), 0),
            1 => do_when_one_param(&params[0]),
            2 => do_when_two_params(&params[0], &params[1]),
            cnt => self.param_count_too_many(self.func(), cnt),
        }
    }

    /// convert [ArcTopicDataValue::Str] to string, otherwise raise error
    pub fn param_to_str<'a>(
        &self,
        param: &'a ArcTopicDataValue,
        param_index: usize,
    ) -> StdR<&'a String> {
        match param {
            ArcTopicDataValue::Str(sub) => Ok(sub.deref()),
            other => return self.param_must_be_str(self.func(), param_index, other),
        }
    }

    /// convert to usize, otherwise raise error
    pub fn param_to_usize(
        &self,
        param: &ArcTopicDataValue,
        none_value: usize,
        param_index: usize,
    ) -> StdR<usize> {
        if let Ok(value) = param.try_to_usize_or_if_none(none_value) {
            Ok(value)
        } else {
            self.param_must_be_num(self.func(), param_index, param)
        }
    }
}
