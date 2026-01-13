use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use elf_base::DateTimeUtils;
use std::ops::Deref;
use std::str::FromStr;

impl ArcTopicDataValue {
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
            Self::None => {
                // #1.1
                another.is_none_or_empty_str()
            }
            Self::Str(one_str) => {
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
            Self::Num(one_decimal) => {
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
            Self::Bool(one_bool) => {
                if let Ok(another_bool) = another.try_to_bool() {
                    // 4.1, 4.2, 4.3, 4.4, 4.5
                    *one_bool == another_bool
                } else {
                    false
                }
            }
            Self::DateTime(one_datetime) => {
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
            Self::Date(one_date) => {
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
            Self::Time(one_time) => {
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
            Self::Map(_) => false,
            // vec is not comparable
            Self::Vec(_) => false,
        }
    }

    /// refer to [is_same_as]
    pub fn is_not_same_as(&self, another: &ArcTopicDataValue) -> bool {
        !self.is_same_as(another)
    }
}
