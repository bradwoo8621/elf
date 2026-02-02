use crate::{ArcTopicDataValue, InMemoryFuncCall, PipelineKernelErrorCode};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_base::{DateTimeUtils, ErrorCode, NumericUtils, StdR, StringUtils, VoidR};
use std::ops::Deref;
use std::sync::Arc;

struct MinmaxState<'a> {
    allow_decimal: bool,
    allow_datetime: bool,
    allow_date: bool,
    allow_time: bool,
    ask_min_value: bool,

    context: &'a Arc<ArcTopicDataValue>,

    func_call: &'a InMemoryFuncCall<'a>,

    str_elements: Vec<Arc<String>>,
    decimal_result: Option<Arc<BigDecimal>>,
    datetime_result: Option<Arc<NaiveDateTime>>,
    date_result: Option<Arc<NaiveDate>>,
    time_result: Option<Arc<NaiveTime>>,
}

impl<'a> MinmaxState<'a> {
    fn build(
        func_call: &'a InMemoryFuncCall<'a>,
        context: &'a Arc<ArcTopicDataValue>,
        allow_decimal: bool,
        allow_datetime: bool,
        allow_date: bool,
        allow_time: bool,
        ask_min_value: bool,
    ) -> Self {
        MinmaxState {
            allow_decimal,
            allow_datetime,
            allow_date,
            allow_time,
            ask_min_value,

            context,

            func_call,

            str_elements: vec![],
            decimal_result: None,
            datetime_result: None,
            date_result: None,
            time_result: None,
        }
    }
}

impl MinmaxState<'_> {
    fn check_indicators(&self) -> VoidR {
        // check allowable indicators
        match (
            self.allow_decimal,
            self.allow_datetime,
            self.allow_date,
            self.allow_time,
        ) {
            (true, false, false, false)
            | (false, true, false, false)
            | (false, false, true, false)
            | (false, false, false, true)
            | (true, true, true, true) => {
                // allowed, do nothing
                Ok(())
            }
            _ => {
                PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
                    "Cannot retrieve[key={}, current={}], \
                    caused by only one of decimal/date/datetime/time is allowed or all 4 types are allowed, \
                    and current is [allow_decimal={}, allow_datetime={}, allow_date={}, allow_time={}].",
                    self.func_call.full_path(),
                    self.func_call.this_path(),
                    self.allow_decimal,
                    self.allow_datetime,
                    self.allow_date,
                    self.allow_time
                ))
            }
        }
    }

    fn func_not_supported<R>(&self) -> StdR<R> {
        self.func_call.func_not_supported(self.context.deref())
    }

    /// - return [ArcTopicDataValue::None] when ask min value,
    /// - return [None]
    fn on_none_element(&self) -> Option<ArcTopicDataValue> {
        if self.ask_min_value {
            Some(ArcTopicDataValue::None)
        } else {
            None
        }
    }

    /// - return [ArcTopicDataValue::None] when ask min value and given string is empty,
    /// - raise error when given string is blank,
    /// - push given string into string elements
    fn on_str_element(&mut self, str: &Arc<String>) -> StdR<Option<ArcTopicDataValue>> {
        if str.is_empty() {
            if self.ask_min_value {
                // empty string treated as none
                Ok(Some(ArcTopicDataValue::None))
            } else {
                // ignore empty string
                Ok(None)
            }
        } else if str.is_blank() {
            // blank string cannot cast to any allowed type, raise error
            self.func_not_supported()
        } else {
            // push to string elements, handle later
            self.str_elements.push(str.clone());
            Ok(None)
        }
    }

    /// compare given decimal value with found min/max result (if exists)
    /// and return true when needs to replace the min/max result
    fn should_replace_decimal_result(&self, decimal: &BigDecimal) -> bool {
        if let Some(found) = self.decimal_result.as_ref() {
            if self.ask_min_value {
                // this value is less than the found one, should replace
                found.deref() > decimal
            } else {
                // this value is greater than the found one, should replace
                found.deref() < decimal
            }
        } else {
            true
        }
    }

    /// - raise error when [allow_decimal] is false,
    /// - set [allow_datetime], [allow_date], [allow_time] to false,
    /// - replace [decimal_result] by given value if no result detected yet,
    /// - replace [decimal_result] by given value when ask min value and given value is less than the found value,
    /// - replace [decimal_result] by given value when ask max value and given value is greater than the found value,
    fn on_decimal_element(&mut self, value: &Arc<BigDecimal>) -> VoidR {
        if self.allow_decimal {
            // temporal types are not allowed anymore
            self.allow_datetime = false;
            self.allow_date = false;
            self.allow_time = false;
            if self.should_replace_decimal_result(value.deref()) {
                self.decimal_result = Some(value.clone());
            }
            Ok(())
        } else {
            // decimal is disallowed
            self.func_not_supported()
        }
    }

    /// compare given datetime value with found min/max result (if exists)
    /// and return true when needs to replace the min/max result
    fn should_replace_datetime_result(&self, datetime: &NaiveDateTime) -> bool {
        if let Some(found) = self.datetime_result.as_ref() {
            if self.ask_min_value {
                // this value is less than the found one, should replace
                found.deref() > datetime
            } else {
                // this value is greater than the found one, should replace
                found.deref() < datetime
            }
        } else {
            true
        }
    }

    // noinspection DuplicatedCode
    /// - raise error when [allow_date] and [allow_datetime] are false,
    /// - set [allow_decimal], [allow_time] to false,
    /// - replace [date_result] by given value if no result detected yet,
    /// - replace [datetime_result] by given value (time part set to 0) if no result detected yet,
    /// - compare given value to [datetime_result] when ask min value,
    ///   - replace [date_result] by given value if given value is less than the found value,
    ///   - replace [datetime_result] by given value (time part set to 0) if given value is less than the found value,
    /// - compare given value to [datetime_result] when ask max value,
    ///   - replace [date_result] by given value if given value is greater than the found value,
    ///   - replace [datetime_result] by given value (time part set to 0) if given value is greater than the found value,
    fn on_date_element(&mut self, date: &Arc<NaiveDate>) -> VoidR {
        if self.allow_date || self.allow_datetime {
            self.allow_decimal = false;
            self.allow_time = false;
            // on cast date to datetime, always use the 00:00:00.000,
            // the found date and datetime are always set together.
            date.and_hms_nano_opt(0, 0, 0, 0).map(|datetime| {
                if self.should_replace_datetime_result(&datetime) {
                    self.date_result = Some(date.clone());
                    self.datetime_result = Some(Arc::new(datetime));
                }
            });

            Ok(())
        } else {
            // date/datetime are disallowed
            self.func_not_supported()
        }
    }

    /// - raise error when [allow_date] and [allow_datetime] are false,
    /// - set [allow_decimal], [allow_time] to false,
    /// - replace [date_result] by given value (truncate the time part) if no result detected yet,
    /// - replace [datetime_result] by given value if no result detected yet,
    /// - compare given value to [datetime_result] when ask min value,
    ///   - replace [date_result] by given value (truncate the time part) if given value is less than the found value,
    ///   - replace [datetime_result] by given value if given value is less than the found value,
    /// - compare given value to [datetime_result] when ask max value,
    ///   - replace [date_result] by given value (truncate the time part) if given value is greater than the found value,
    ///   - replace [datetime_result] by given value if given value is greater than the found value,
    fn on_datetime_element(&mut self, datetime: &Arc<NaiveDateTime>) -> VoidR {
        if self.allow_date || self.allow_datetime {
            self.allow_decimal = false;
            self.allow_time = false;
            // the found date and datetime are always set together.
            if self.should_replace_datetime_result(datetime.deref()) {
                self.date_result = Some(Arc::new(datetime.date()));
                self.datetime_result = Some(datetime.clone());
            }
            Ok(())
        } else {
            // date/datetime are disallowed
            self.func_not_supported()
        }
    }

    /// compare given time value with found min/max result (if exists)
    /// and return true when needs to replace the min/max result
    fn should_replace_time_result(&self, time: &NaiveTime) -> bool {
        if let Some(found) = self.time_result.as_ref() {
            if self.ask_min_value {
                // this value is less than the found one, should replace
                found.deref() > time
            } else {
                // this value is greater than the found one, should replace
                found.deref() < time
            }
        } else {
            true
        }
    }

    /// - raise error when [allow_time] is false,
    /// - set [allow_decimal], [allow_datetime], [allow_date] to false,
    /// - replace [time_result] by given value if no result detected yet,
    /// - replace [time_result] by given value when ask min value and given value is less than the found value,
    /// - replace [time_result] by given value when ask max value and given value is greater than the found value,
    fn on_time_element(&mut self, time: &Arc<NaiveTime>) -> VoidR {
        if self.allow_time {
            self.allow_decimal = false;
            self.allow_datetime = false;
            self.allow_date = false;
            if self.should_replace_time_result(time.deref()) {
                self.time_result = Some(time.clone());
            }
            Ok(())
        } else {
            // time is disallowed
            self.func_not_supported()
        }
    }

    fn find_on_types(
        &mut self,
        vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
    ) -> StdR<Option<ArcTopicDataValue>> {
        for element in vec.iter() {
            match element.deref() {
                ArcTopicDataValue::None => {
                    if let Some(found) = self.on_none_element() {
                        return Ok(Some(found));
                    }
                }
                ArcTopicDataValue::Str(str) => {
                    if let Some(found) = self.on_str_element(str)? {
                        return Ok(Some(found));
                    }
                }
                ArcTopicDataValue::Num(num) => self.on_decimal_element(num)?,
                ArcTopicDataValue::Date(date) => self.on_date_element(date)?,
                ArcTopicDataValue::DateTime(datetime) => self.on_datetime_element(datetime)?,
                ArcTopicDataValue::Time(time) => self.on_time_element(time)?,
                // bool, vec, map don't support minmax
                ArcTopicDataValue::Bool(_)
                | ArcTopicDataValue::Vec(_)
                | ArcTopicDataValue::Map(_) => {
                    return self.func_not_supported();
                }
            }
        }

        Ok(None)
    }

    /// the data type to be found has been determined as decimal.
    /// attempt to convert string elements to decimal, and obtain the min/max values.
    fn continue_find_decimal_on_str_elements(&mut self) -> VoidR {
        for element in self.str_elements.iter() {
            if let Ok(decimal) = element.to_decimal() {
                if self.should_replace_decimal_result(&decimal) {
                    self.decimal_result = Some(Arc::new(decimal));
                }
            } else {
                return self.func_not_supported();
            }
        }

        Ok(())
    }

    // noinspection DuplicatedCode
    /// the data type to be found has been determined as date.
    /// attempt to convert string elements to date, and obtain the min/max values.
    /// always compare to the found datetime
    fn continue_find_date_and_datetime_on_str_elements(&mut self) -> VoidR {
        for element in self.str_elements.iter() {
            if let Ok(datetime) = element.to_datetime_loose() {
                // the found date and datetime are always set together.
                if self.should_replace_datetime_result(&datetime) {
                    self.date_result = Some(Arc::new(datetime.date()));
                    self.datetime_result = Some(Arc::new(datetime));
                }
            } else {
                return self.func_not_supported();
            }
        }

        Ok(())
    }

    fn continue_find_time_on_str_elements(&mut self) -> VoidR {
        for element in self.str_elements.iter() {
            if let Ok(time) = element.to_time() {
                if self.should_replace_time_result(&time) {
                    self.time_result = Some(Arc::new(time));
                }
            } else {
                return self.func_not_supported();
            }
        }

        Ok(())
    }

    // noinspection DuplicatedCode
    /// so far, the logic above may not have obtained any min/max values based on type.
    /// therefore, when analyzing string values,
    /// they could still be any of the following types: decimal, date, datetime, or time.
    /// moreover, since the system converts date/time by removing noise characters (characters other than 0-9 and +),
    /// this could lead to a number being incorrectly recognized as a date/time type, or oppositely.
    /// hence, the following analysis needs to prioritize decimal parsing.
    /// - if all content can be identified as decimal, the possibility of date/datetime/time should be ignored,
    /// - if any element can be identified as date/datetime/time,
    ///   and it cannot be cast to decimal, the possibility of decimal should be ignored,
    /// - priority of date/datetime is higher than time,
    fn continue_find_any_on_str_elements(&mut self) -> VoidR {
        let mut hold_elements = vec![];
        for element in self.str_elements.iter() {
            if self.allow_decimal
                && let Ok(decimal) = element.to_decimal()
            {
                // hold this element, many some after will be date/datetime/time
                // so this element needs to be reparsed
                hold_elements.push(element);
                // decimal has top priority
                if self.should_replace_decimal_result(&decimal) {
                    self.decimal_result = Some(Arc::new(decimal));
                }
            } else if (self.allow_date || self.allow_datetime)
                && let Ok(mut datetime) = element.to_datetime_loose()
            {
                // check the hold elements
                if !hold_elements.is_empty() {
                    for hold_element in hold_elements.iter() {
                        if let Ok(hold_datetime) = hold_element.to_datetime_loose() {
                            if self.ask_min_value {
                                if datetime > hold_datetime {
                                    datetime = hold_datetime;
                                }
                            } else if datetime < hold_datetime {
                                datetime = hold_datetime;
                            }
                        } else {
                            // hold element cannot be cast to datetime, raise error
                            return self.func_not_supported();
                        }
                    }
                    // and clear hold elements
                    hold_elements.clear();
                }

                self.allow_decimal = false;
                self.allow_time = false;
                if self.should_replace_datetime_result(&datetime) {
                    self.date_result = Some(Arc::new(datetime.date()));
                    self.datetime_result = Some(Arc::new(datetime));
                }
            } else if self.allow_time
                && let Ok(mut time) = element.to_time()
            {
                // check the hold elements
                if !hold_elements.is_empty() {
                    for hold_element in hold_elements.iter() {
                        if let Ok(hold_time) = hold_element.to_time() {
                            if self.ask_min_value {
                                if time > hold_time {
                                    time = hold_time;
                                }
                            } else if time < hold_time {
                                time = hold_time;
                            }
                        } else {
                            // hold element cannot be cast to time, raise error
                            return self.func_not_supported();
                        }
                    }
                    // and clear hold elements
                    hold_elements.clear();
                }

                self.allow_decimal = false;
                self.allow_date = false;
                self.allow_datetime = false;
                if self.should_replace_time_result(&time) {
                    self.time_result = Some(Arc::new(time));
                }
            } else {
                // this string value cannot be cast to any of decimal/date/datetime/time
                // raise error
                return self.func_not_supported();
            }
        }

        // everything is ok, then make sure if date/datetime allowed, return datetime
        if self.allow_date {
            self.allow_date = false;
        }

        Ok(())
    }

    /// find min/max on string elements, the on types finding is accomplished
    fn continue_find_on_str_elements(&mut self) -> VoidR {
        if self.str_elements.is_empty() {
            return Ok(());
        }

        match (
            self.allow_decimal,
            self.allow_datetime,
            self.allow_date,
            self.allow_time,
        ) {
            // allow decimal only
            (true, false, false, false) => self.continue_find_decimal_on_str_elements(),
            // allow datetime, allow date,
            // or allow both (allow indicators changed during on types finding)
            (false, true, _, false) | (false, _, true, false) => {
                self.continue_find_date_and_datetime_on_str_elements()
            }
            // allow time only
            (false, false, false, true) => self.continue_find_time_on_str_elements(),
            // allow all types
            (true, true, true, true) => self.continue_find_any_on_str_elements(),
            // never happen, simply raise error
            _ => self.func_not_supported(),
        }
    }

    /// return the found min/max value
    fn ask_found_value(self) -> StdR<Arc<ArcTopicDataValue>> {
        match (
            self.allow_decimal,
            self.allow_datetime,
            self.allow_date,
            self.allow_time,
        ) {
            (true, false, false, false) => {
                if let Some(decimal) = self.decimal_result {
                    Ok(Arc::new(ArcTopicDataValue::Num(decimal)))
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            (false, true, false, false) => {
                if let Some(datetime) = self.datetime_result {
                    Ok(Arc::new(ArcTopicDataValue::DateTime(datetime)))
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            (false, false, true, false) => {
                if let Some(date) = self.date_result {
                    Ok(Arc::new(ArcTopicDataValue::Date(date)))
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            (false, false, false, true) => {
                if let Some(time) = self.time_result {
                    Ok(Arc::new(ArcTopicDataValue::Time(time)))
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            (false, false, false, false) => {
                // no typed value found, and there is no error raised,
                // means all values are none
                Ok(Arc::new(ArcTopicDataValue::None))
            }
            // raise error, neve happen
            _ => self.func_not_supported(),
        }
    }

    /// find min/max value from context
    fn find(mut self) -> StdR<Arc<ArcTopicDataValue>> {
        match self.context.deref() {
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                }

                if let Some(found) = self.find_on_types(vec)? {
                    return Ok(Arc::new(found));
                }

                self.continue_find_on_str_elements()?;

                self.ask_found_value()
            }
            _ => self.func_not_supported(),
        }
    }
}

impl InMemoryFuncCall<'_> {
    /// - [VariablePredefineFunctions::Min]
    /// - [VariablePredefineFunctions::MinNum]
    /// - [VariablePredefineFunctions::MinDatetime], [VariablePredefineFunctions::MinDt]
    /// - [VariablePredefineFunctions::MinDate]
    /// - [VariablePredefineFunctions::MinTime]
    /// - [VariablePredefineFunctions::Max]
    /// - [VariablePredefineFunctions::MaxNum]
    /// - [VariablePredefineFunctions::MaxDatetime], [VariablePredefineFunctions::MaxDt]
    /// - [VariablePredefineFunctions::MaxDate]
    /// - [VariablePredefineFunctions::MaxTime]
    ///
    /// compute min/max value of given vec. rules depend on which value to ask.
    /// - for asking min value, none and empty string treated as min value,
    /// - for asking max value, ignore none and empty string.
    pub fn resolve_minmax_of_vec(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
        allow_decimal: bool,
        allow_datetime: bool,
        allow_date: bool,
        allow_time: bool,
        ask_min_value: bool,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || {
            let state = MinmaxState::build(
                &self,
                &context,
                allow_decimal,
                allow_datetime,
                allow_date,
                allow_time,
                ask_min_value,
            );
            state.check_indicators()?;
            state.find()
        })
    }
}
