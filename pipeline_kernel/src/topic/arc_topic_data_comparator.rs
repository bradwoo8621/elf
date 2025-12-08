use crate::{ArcTopicDataValue, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, One, Zero};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use watchmen_model::{DateTimeUtils, NumericUtils, StdErrorCode, StdR};

impl ArcTopicDataValue {
    /// check itself is [None] or not
    pub fn is_none(&self) -> bool {
        match self {
            ArcTopicDataValue::None => true,
            _ => false,
        }
    }

    /// check itself is string or not
    pub fn is_str(&self) -> Result<&String, &Self> {
        match self {
            ArcTopicDataValue::Str(s) => Ok(s),
            _ => Err(self),
        }
    }

    /// check itself is bool or not
    pub fn is_bool(&self) -> Result<&bool, &Self> {
        match self {
            ArcTopicDataValue::Bool(b) => Ok(b),
            _ => Err(self),
        }
    }

    /// try to cast itself to bool
    /// boolean -> bool
    /// string [1, true, t, yes, y] -> true
    /// string [0, false, f, no, n] -> false
    /// decimal [1] -> true
    /// decimal [0] -> false
    /// others -> cannot to bool, none
    pub fn try_to_bool(&self) -> Result<bool, &Self> {
        match self {
            ArcTopicDataValue::Bool(b) => Ok(*b),
            ArcTopicDataValue::Str(s) => match s.to_ascii_lowercase().as_str() {
                "1" | "true" | "t" | "yes" | "y" => Ok(true),
                "0" | "false" | "f" | "no" | "n" => Ok(false),
                _ => Err(self),
            },
            ArcTopicDataValue::Num(n) => {
                if n.is_one() {
                    Ok(true)
                } else if n.is_zero() {
                    Ok(false)
                } else {
                    Err(self)
                }
            }
            _ => Err(self),
        }
    }

    /// check itself is decimal or not
    pub fn is_num(&self) -> Result<&BigDecimal, &Self> {
        match self {
            ArcTopicDataValue::Num(n) => Ok(n),
            _ => Err(self),
        }
    }

    /// check itself is datetime or not
    pub fn is_datetime(&self) -> Result<&NaiveDateTime, &Self> {
        match self {
            ArcTopicDataValue::DateTime(dt) => Ok(dt),
            _ => Err(self),
        }
    }

    /// check itself is date or not
    pub fn is_date(&self) -> Result<&NaiveDate, &Self> {
        match self {
            ArcTopicDataValue::Date(d) => Ok(d),
            _ => Err(self),
        }
    }

    /// check itself is time or not
    pub fn is_time(&self) -> Result<&NaiveTime, &Self> {
        match self {
            ArcTopicDataValue::Time(t) => Ok(t),
            _ => Err(self),
        }
    }

    /// check itself is date/time/datetime or not
    pub fn is_datetime_related(&self) -> bool {
        match self {
            ArcTopicDataValue::Date(_) => true,
            ArcTopicDataValue::DateTime(_) => true,
            ArcTopicDataValue::Time(_) => true,
            _ => false,
        }
    }

    /// [None], [Empty Str], [Empty Map], [Empty Vec] -> true,
    /// otherwise: false
    pub fn is_empty(&self) -> bool {
        match self {
            ArcTopicDataValue::None => true,
            ArcTopicDataValue::Str(v) => v.len() == 0,
            ArcTopicDataValue::Map(v) => v.is_empty(),
            ArcTopicDataValue::Vec(v) => v.is_empty(),
            _ => false,
        }
    }

    /// [None], [Empty Str], [Empty Map], [Empty Vec] -> false,
    /// otherwise: true
    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// [None], [Empty Str] -> true,
    /// otherwise: false
    pub fn is_none_or_empty_str(&self) -> bool {
        match self {
            ArcTopicDataValue::None => true,
            ArcTopicDataValue::Str(v) => v.len() == 0,
            _ => false,
        }
    }

    /// same as when
    /// 1. one is none:
    ///    - 1.1. another is none or empty string,
    /// 2. one is string:
    ///    - 2.1. another is string, equals,
    ///    - 2.2. one is empty string, another is none or empty string,
    ///    - 2.3. another is boolean true, one is [1, t, true, y, yes],
    ///    - 2.4. another is boolean false, one is [0, f, false, n, no],
    ///    - 2.5. another is decimal, equals one to decimal,
    ///    - 2.6. another is datetime, equals one to datetime or date, both truncate time part,
    ///    - 2.7. another is date, equals one to datetime (truncate time part) or date,
    ///    - 2.8. another is time, equals one to time,
    /// 3. one is decimal:
    ///    - 3.1. another is decimal, equals,
    ///    - 3.2. another is boolean true, one is [1],
    ///    - 3.3. another is boolean false, one is [0],
    ///    - 3.4. another is string, equals another to decimal,
    /// 4. one is boolean:
    ///    - 4.1. another is boolean, equals,
    ///    - 4.2. one is true, another is string [1, t, true, y, yes],
    ///    - 4.3. one is false, another is string [0, f, false, n, no],
    ///    - 4.4. one is true, another is decimal [1],
    ///    - 4.5. one is false, another is decimal [0],
    /// 5. one is datetime:
    ///    - 5.1. another is datetime, both truncate time part, equals,
    ///    - 5.2. another is date, truncate one's time part, equals,
    ///    - 5.3. another is string, equals another to datetime or date, both truncate time part,
    /// 6. one is date:
    ///    - 6.1. another is datetime, truncate another's time part, equals,
    ///    - 6.2. another is date, equals,
    ///    - 6.3. another is string, equals another to datetime (truncate time part) or date,
    /// 7. one is time:
    ///    - 7.1. another is time, equals,
    ///    - 7.2. another is string, equals another to time
    pub fn is_same_as(&self, another: &ArcTopicDataValue) -> bool {
        match self {
            ArcTopicDataValue::None => {
                // #1.1
                another.is_none_or_empty_str()
            }
            ArcTopicDataValue::Str(one_str) => {
                if let Ok(another_str) = another.is_str() {
                    // 2.1
                    one_str.deref() == another_str
                } else if one_str.len() == 0 {
                    // 2.2
                    another.is_none_or_empty_str()
                } else if let Ok(another_bool) = another.is_bool() {
                    // 2.3, 2.4
                    if let Ok(one_bool) = self.try_to_bool() {
                        one_bool == *another_bool
                    } else {
                        false
                    }
                } else if let Ok(another_decimal) = another.is_num() {
                    // 2.5
                    if let Ok(one_decimal) = BigDecimal::from_str(one_str.as_str()) {
                        &one_decimal == another_decimal
                    } else {
                        false
                    }
                } else if let Ok(another_datetime) = another.is_datetime() {
                    // 2.6
                    if let Ok(one_date) = one_str.to_date_loose() {
                        one_date == another_datetime.date()
                    } else {
                        false
                    }
                } else if let Ok(another_date) = another.is_date() {
                    // 2.7
                    if let Ok(one_date) = one_str.to_date_loose() {
                        one_date == *another_date
                    } else {
                        false
                    }
                } else if let Ok(another_time) = another.is_time() {
                    // 2.8
                    if let Ok(one_time) = one_str.to_time() {
                        one_time == *another_time
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            ArcTopicDataValue::Num(one_decimal) => {
                if let Ok(another_decimal) = another.is_num() {
                    // 3.1
                    one_decimal.deref() == another_decimal
                } else if let Ok(another_bool) = another.is_bool() {
                    // 3.2, 3.3
                    if let Ok(one_bool) = self.try_to_bool() {
                        one_bool == *another_bool
                    } else {
                        false
                    }
                } else if let Ok(another_str) = another.is_str() {
                    // 3.4
                    if let Ok(another_decimal) = BigDecimal::from_str(another_str.as_str()) {
                        one_decimal.deref() == &another_decimal
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            ArcTopicDataValue::Bool(one_bool) => {
                if let Ok(another_bool) = another.try_to_bool() {
                    // 4.1, 4.2, 4.3, 4.4, 4.5
                    *one_bool == another_bool
                } else {
                    false
                }
            }
            ArcTopicDataValue::DateTime(one_datetime) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 5.1
                    one_datetime.date() == another_datetime.date()
                } else if let Ok(another_date) = another.is_date() {
                    // 5.2
                    one_datetime.date() == *another_date
                } else if let Ok(another_str) = another.is_str() {
                    // 5.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        one_datetime.date() == another_date
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            ArcTopicDataValue::Date(one_date) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 6.1
                    *one_date.deref() == another_datetime.date()
                } else if let Ok(another_date) = another.is_date() {
                    // 6.2
                    *one_date.deref() == *another_date
                } else if let Ok(another_str) = another.is_str() {
                    // 6.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        *one_date.deref() == another_date
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            ArcTopicDataValue::Time(one_time) => {
                if let Ok(another_time) = another.is_time() {
                    // 7.1
                    *one_time.deref() == *another_time
                } else if let Ok(another_str) = another.is_str() {
                    // 7.2
                    if let Ok(another_time) = another_str.to_time() {
                        *one_time.deref() == another_time
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            // map is not comparable
            ArcTopicDataValue::Map(_) => false,
            // vec is not comparable
            ArcTopicDataValue::Vec(_) => false,
        }
    }

    /// refer to [is_same_as]
    pub fn is_not_same_as(&self, another: &ArcTopicDataValue) -> bool {
        !self.is_same_as(another)
    }

    pub(crate) fn display_in_error(value: &ArcTopicDataValue) -> String {
        match value {
            ArcTopicDataValue::None => String::from("none"),
            ArcTopicDataValue::Str(v) => v.to_string(),
            ArcTopicDataValue::Num(v) => v.to_plain_string(),
            ArcTopicDataValue::Bool(v) => String::from(if *v { "true" } else { "false" }),
            ArcTopicDataValue::Date(v) => v.to_string(),
            ArcTopicDataValue::Time(v) => v.to_string(),
            ArcTopicDataValue::DateTime(v) => v.to_string(),
            ArcTopicDataValue::Map(_) => String::from("map"),
            ArcTopicDataValue::Vec(_) => String::from("vec"),
        }
    }

    fn must_compare_between_num_or_datetime<R>(&self, another: &ArcTopicDataValue) -> StdR<R> {
        PipelineKernelErrorCode::ValuesNotComparable.msg(
            format!("Comparison of [none|str|decimal|date|time|datetime] are supported, current are [one={:?}, another={:?}].",
                    Self::display_in_error(self), Self::display_in_error(another)), )
    }

    /// less than when
    /// 1. one is none:
    ///    - 1.1. another is none -> false,
    ///    - 1.2. another is decimal or datetime related -> true,
    ///    - 1.3. error,
    /// 2. one is string:
    ///    - 2.1. another is decimal, more or equals one to decimal,
    ///    - 2.2. another is time, more or equals one to time,
    ///    - 2.3. another is date, more or equals one to datetime (truncate time part) or date,
    ///    - 2.4. another is datetime, more or equals one to datetime or date, both truncate time part,
    ///    - 2.5. error, note [string cannot compare to string],
    /// 3. one is decimal:
    ///    - 3.1. another is decimal, less than,
    ///    - 3.2. another is string, less than another to decimal,
    ///    - 3.3. error,
    /// 4. one is datetime:
    ///    - 4.1. another is datetime, truncate both time part, less than,
    ///    - 4.2. another is date, truncate one's time part, less than
    ///    - 4.3. another is string, less than another to datetime (truncate both time part) or date,
    ///    - 4.4. error,
    /// 5. one is date:
    ///    - 5.1. another is datetime, truncate other's time part, less than,
    ///    - 5.2. another is date, less than,
    ///    - 5.3. another is string, less than another to datetime (truncate other's time part) or date,
    ///    - 5.4. error,
    /// 6. one is time:
    ///    - 6.1. another is time, less than,
    ///    - 6.2. another is string, less than another to time,
    ///    - 6.3. error,
    /// 7. error.
    pub fn is_less_than(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match self {
            ArcTopicDataValue::None => {
                if another.is_none() {
                    // 1.1
                    Ok(false)
                } else if another.is_num().is_ok() || another.is_datetime_related() {
                    // 1.2
                    Ok(true)
                } else {
                    // 1.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Str(one_value) => {
                if let Ok(another_decimal) = another.is_num() {
                    // 2.1
                    if let Ok(one_decimal) = one_value.to_decimal() {
                        Ok(one_decimal < *another_decimal)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_datetime) = another.is_datetime() {
                    // 2.4
                    if let Ok(one_date) = one_value.to_date_loose() {
                        Ok(one_date < another_datetime.date())
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_date) = another.is_date() {
                    // 2.3
                    if let Ok(one_date) = one_value.to_date_loose() {
                        Ok(one_date < *another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_time) = another.is_time() {
                    // 2.2
                    if let Ok(one_time) = one_value.to_time() {
                        Ok(one_time < *another_time)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 2.5
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Num(one_decimal) => {
                if let Ok(another_decimal) = another.is_num() {
                    // 3.1
                    Ok(one_decimal.deref() < another_decimal)
                } else if let Ok(another_str) = another.is_str() {
                    // 3.2
                    if let Ok(another_decimal) = another_str.to_decimal() {
                        Ok(*one_decimal.deref() < another_decimal)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 3.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Bool(_) => self.must_compare_between_num_or_datetime(another),
            ArcTopicDataValue::DateTime(one_datetime) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 4.1
                    Ok(one_datetime.date() < another_datetime.date())
                } else if let Ok(another_date) = another.is_date() {
                    // 4.2
                    Ok(one_datetime.date() < *another_date)
                } else if let Ok(another_str) = another.is_str() {
                    // 4.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        Ok(one_datetime.date() < another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 4.4
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Date(one_date) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 5.1
                    Ok(*one_date.deref() < another_datetime.date())
                } else if let Ok(another_date) = another.is_date() {
                    // 5.2
                    Ok(*one_date.deref() < *another_date)
                } else if let Ok(another_str) = another.is_str() {
                    // 5.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        Ok(*one_date.deref() < another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 5.4
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Time(one_time) => {
                if let Ok(another_time) = another.is_time() {
                    // 6.1
                    Ok(*one_time.deref() < *another_time)
                } else if let Ok(another_str) = another.is_str() {
                    // 6.2
                    if let Ok(another_time) = another_str.to_time() {
                        Ok(*one_time.deref() < another_time)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 6.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            // map is not comparable
            ArcTopicDataValue::Map(_) => self.must_compare_between_num_or_datetime(another),
            // vec is not comparable
            ArcTopicDataValue::Vec(_) => self.must_compare_between_num_or_datetime(another),
        }
    }

    /// refer to [is_less_than]
    pub fn is_less_than_or_equals(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        self.is_more_than(another).map(|b| !b)
    }

    /// refer to [is_less_than]
    pub fn is_more_than(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match self {
            ArcTopicDataValue::None => {
                if another.is_none() || another.is_num().is_ok() || another.is_datetime_related() {
                    // 1.1, 1.2
                    Ok(false)
                } else {
                    // 1.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Str(one_str) => {
                if let Ok(another_decimal) = another.is_num() {
                    // 2.1
                    if let Ok(one_decimal) = one_str.to_decimal() {
                        Ok(one_decimal > *another_decimal)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_datetime) = another.is_datetime() {
                    // 2.4
                    if let Ok(one_date) = one_str.to_date_loose() {
                        Ok(one_date > another_datetime.date())
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_date) = another.is_date() {
                    // 2.3
                    if let Ok(one_date) = one_str.to_date_loose() {
                        Ok(one_date > *another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else if let Ok(another_time) = another.is_time() {
                    // 2.2
                    if let Ok(one_time) = one_str.to_time() {
                        Ok(one_time > *another_time)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 2.5
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Num(one_decimal) => {
                if let Ok(another_decimal) = another.is_num() {
                    // 3.1
                    Ok(one_decimal.deref() > another_decimal)
                } else if let Ok(another_str) = another.is_str() {
                    // 3.2
                    if let Ok(another_decimal) = another_str.to_decimal() {
                        Ok(*one_decimal.deref() > another_decimal)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 3.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Bool(_) => self.must_compare_between_num_or_datetime(another),
            ArcTopicDataValue::DateTime(one_datetime) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 4.1
                    Ok(one_datetime.date() > another_datetime.date())
                } else if let Ok(another_date) = another.is_date() {
                    // 4.2
                    Ok(one_datetime.date() > *another_date)
                } else if let Ok(another_str) = another.is_str() {
                    // 4.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        Ok(one_datetime.date() > another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 4.4
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Date(one_date) => {
                if let Ok(another_datetime) = another.is_datetime() {
                    // 5.1
                    Ok(*one_date.deref() > another_datetime.date())
                } else if let Ok(another_date) = another.is_date() {
                    // 5.2
                    Ok(*one_date.deref() > *another_date)
                } else if let Ok(another_str) = another.is_str() {
                    // 5.3
                    if let Ok(another_date) = another_str.to_date_loose() {
                        Ok(*one_date.deref() > another_date)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 4.4
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            ArcTopicDataValue::Time(one_time) => {
                if let Ok(another_time) = another.is_time() {
                    // 6.1
                    Ok(*one_time.deref() > *another_time)
                } else if let Ok(another_str) = another.is_str() {
                    // 6.2
                    if let Ok(another_time) = another_str.to_time() {
                        Ok(*one_time.deref() > another_time)
                    } else {
                        self.must_compare_between_num_or_datetime(another)
                    }
                } else {
                    // 6.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            // map is not comparable
            ArcTopicDataValue::Map(_) => self.must_compare_between_num_or_datetime(another),
            // vec is not comparable
            ArcTopicDataValue::Vec(_) => self.must_compare_between_num_or_datetime(another),
        }
    }

    /// refer to [is_less_than]
    pub fn is_more_than_or_equals(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        self.is_less_than(another).map(|b| !b)
    }

    /// in when
    /// 1. another is none -> false
    /// 2. another is vec, check if there is any same,
    /// 3. another is string, split with comma, check if there is any same,
    /// 4. error.
    pub fn is_in(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match another {
            ArcTopicDataValue::None => Ok(false),
            ArcTopicDataValue::Vec(another_vec) => match self {
                ArcTopicDataValue::Vec(_) => Ok(false),
                ArcTopicDataValue::Map(_) => Ok(false),
                // same as any element in vec
                _ => Ok(another_vec.iter().any(|another_value| self.is_same_as(another_value)))
            },
            ArcTopicDataValue::Str(another_str) => match self {
                ArcTopicDataValue::Vec(_) => Ok(false),
                ArcTopicDataValue::Map(_) => Ok(false),
                _ => Ok(another_str
                    .split(',')
                    .into_iter()
                    .map(|s| ArcTopicDataValue::Str(Arc::new(s.to_string())))
                    .any(|another_value| self.is_same_as(&another_value)))
            }
            _ => PipelineKernelErrorCode::ValuesNotComparable.msg(
                format!("Comparison of [none|str|decimal|date|time|datetime] are supported, current are [one={:?}, another={:?}].",
                        Self::display_in_error(self), Self::display_in_error(another)))
        }
    }

    /// refer to [is_in].
    /// note that none is not in none.
    pub fn is_not_in(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        self.is_in(another).map(|b| !b)
    }
}
