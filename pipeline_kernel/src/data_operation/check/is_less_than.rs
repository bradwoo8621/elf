use crate::ArcTopicDataValue;
use elf_base::{DateTimeUtils, NumericUtils, StdR};
use std::ops::Deref;

impl ArcTopicDataValue {
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
    ///
    /// Note according to #4.1 and #5.1, for datetime type, the time part is not involved in the value comparison.
    /// Therefore, it is possible that a situation occurs where,
    /// for example, "2025-12-09 11:00:00" [is not less than] "2025-12-09 12:00:00".
    pub fn is_less_than(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match self {
            Self::None => {
                if another.is_none_or_empty_str() {
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
            Self::Str(one_value) => {
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
            Self::Num(one_decimal) => {
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
            Self::Bool(_) => self.must_compare_between_num_or_datetime(another),
            Self::DateTime(one_datetime) => {
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
            Self::Date(one_date) => {
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
            Self::Time(one_time) => {
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
            Self::Map(_) => self.must_compare_between_num_or_datetime(another),
            // vec is not comparable
            Self::Vec(_) => self.must_compare_between_num_or_datetime(another),
        }
    }
}
