use crate::{ArcTopicDataValue, MinmaxComparator};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::sync::Arc;
use watchmen_model::{DateTimeUtils, NumericUtils, StdErr, StdR};

#[derive(Debug)]
pub enum MinmaxCompare {
    Less,
    Greater,
}

#[derive(Debug)]
pub struct MinmaxState {
    pub compare: MinmaxCompare,

    pub has_decimal: bool,
    pub min_decimal: Option<Arc<BigDecimal>>,
    pub has_datetime: bool,
    pub min_datetime: Option<Arc<NaiveDateTime>>,
    pub has_date: bool,
    pub min_date: Option<Arc<NaiveDate>>,
    pub has_time: bool,
    pub min_time: Option<Arc<NaiveTime>>,

    pub string_elements: Vec<Arc<String>>,
}

/// creator
impl MinmaxState {
    pub fn new(compare: MinmaxCompare) -> Self {
        MinmaxState {
            compare,

            has_decimal: false,
            min_decimal: None,
            has_datetime: false,
            min_datetime: None,
            has_date: false,
            min_date: None,
            has_time: false,
            min_time: None,

            string_elements: vec![],
        }
    }
}

/// exchange
impl MinmaxState {
    pub fn exchange_with_decimal<NotSupport>(
        &mut self,
        decimal: &Arc<BigDecimal>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_datetime || self.has_date || self.has_time {
            Err(not_support())
        } else {
            self.has_decimal = true;
            self.min_decimal.exchange_if(decimal, &self.compare);
            Ok(true)
        }
    }

    pub fn exchange_with_datetime<NotSupport>(
        &mut self,
        datetime: &Arc<NaiveDateTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_datetime = true;
            self.min_datetime.exchange_if(datetime, &self.compare);
            // datetime also can be compared with date
            self.min_date
                .exchange_if(&Arc::new(datetime.date()), &self.compare);
            Ok(true)
        }
    }

    pub fn exchange_with_date<NotSupport>(
        &mut self,
        date: &Arc<NaiveDate>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_time {
            Err(not_support())
        } else {
            self.has_date = true;
            self.min_date.exchange_if(date, &self.compare);
            Ok(true)
        }
    }

    pub fn exchange_with_time<NotSupport>(
        &mut self,
        time: &Arc<NaiveTime>,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal || self.has_datetime || self.has_date {
            Err(not_support())
        } else {
            self.has_time = true;
            self.min_time.exchange_if(time, &self.compare);
            Ok(true)
        }
    }
}

/// string elements process
impl MinmaxState {
    pub fn with_decimal_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        for value in self.string_elements.clone().iter() {
            if let Ok(decimal) = value.to_decimal() {
                self.exchange_with_decimal(&Arc::new(decimal), not_support)?;
            } else {
                return Err(not_support());
            }
        }
        Ok(true)
    }

    pub fn with_date_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        for value in self.string_elements.clone().iter() {
            if let Ok(date) = value.to_date_loose() {
                self.exchange_with_date(&Arc::new(date), not_support)?;
            } else {
                return Err(not_support());
            }
        }
        Ok(true)
    }

    pub fn with_datetime_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        let mut downgrade_to_date = false;
        for value in self.string_elements.clone().iter() {
            if !downgrade_to_date && let Ok(datetime) = value.to_datetime() {
                self.exchange_with_datetime(&Arc::new(datetime), not_support)?;
            } else if let Ok(date) = value.to_date() {
                downgrade_to_date = true;
                self.exchange_with_date(&Arc::new(date), not_support)?;
            } else {
                return Err(not_support());
            }
        }
        Ok(true)
    }

    pub fn with_time_by_string_elements<NotSupport>(
        &mut self,
        not_support: &NotSupport,
    ) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        for value in self.string_elements.clone().iter() {
            if let Ok(time) = value.to_time() {
                self.exchange_with_time(&Arc::new(time), not_support)?;
            } else {
                return Err(not_support());
            }
        }
        Ok(true)
    }

    /// false means not supported detected
    pub fn with_string_elements<NotSupport>(&mut self, not_support: &NotSupport) -> StdR<bool>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.string_elements.len() == 0 {
            return Ok(true);
        }

        let mut handled = if self.has_decimal {
            self.with_decimal_by_string_elements(not_support)?
        } else {
            false
        };
        if !handled {
            handled = if self.has_date {
                self.with_date_by_string_elements(not_support)?
            } else {
                false
            }
        }
        if !handled {
            handled = if self.has_datetime {
                self.with_datetime_by_string_elements(not_support)?
            } else {
                false
            }
        }
        if !handled {
            handled = if self.has_time {
                self.with_time_by_string_elements(not_support)?
            } else {
                false
            }
        }
        if !handled {
            // 1. process when none of the known types including decimal, datetime, date, time, or none is detected.
            //    - error when not support detected
            //    - true when handled
            // 2. false when one of the known types detected.
            //
            // If there are no separators in time data (datetime/date/time), it may be recognized as a decimal.
            // Therefore, in the process of automatically recognizing strings,
            // it's impossible to avoid the possibility of misrecognition.
            // In this implementation, decimal has the highest priority for recognition,
            // followed by datetime, date, and time in sequence.
            // Meanwhile, if a date is recognized, the previously recognized datetime will be automatically downgraded to a date.
            for value in self.string_elements.clone().iter() {
                if !(self.has_datetime || self.has_date || self.has_time) {
                    if let Ok(decimal) = value.to_decimal() {
                        self.exchange_with_decimal(&Arc::new(decimal), not_support)?;
                        continue;
                    }
                }
                if !(self.has_decimal || self.has_time) {
                    if !self.has_date
                        && let Ok(datetime) = value.to_datetime()
                    {
                        self.exchange_with_datetime(&Arc::new(datetime), not_support)?;
                        continue;
                    } else if let Ok(date) = value.to_date_loose() {
                        self.exchange_with_date(&Arc::new(date), not_support)?;
                        continue;
                    }
                }
                if !(self.has_decimal || self.has_datetime || self.has_date) {
                    if let Ok(time) = value.to_time() {
                        self.exchange_with_time(&Arc::new(time), not_support)?;
                        continue;
                    }
                }
            }
        }
        Ok(true)
    }
}

/// get result
impl MinmaxState {
    pub(crate) fn get_result<NotSupport>(
        &self,
        not_support: &NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        if self.has_decimal {
            Ok(Arc::new(ArcTopicDataValue::Num(
                self.min_decimal.as_ref().unwrap().clone(),
            )))
        } else if self.has_date {
            Ok(Arc::new(ArcTopicDataValue::Date(
                self.min_date.as_ref().unwrap().clone(),
            )))
        } else if self.has_datetime {
            Ok(Arc::new(ArcTopicDataValue::DateTime(
                self.min_datetime.as_ref().unwrap().clone(),
            )))
        } else if self.has_time {
            Ok(Arc::new(ArcTopicDataValue::Time(
                self.min_time.as_ref().unwrap().clone(),
            )))
        } else {
            // no decimal/datetime/date/time/string,
            // function is not supported, since don't know how to compare
            Err(not_support())
        }
    }
}
