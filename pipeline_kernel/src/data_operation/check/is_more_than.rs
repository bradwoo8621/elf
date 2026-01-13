use crate::ArcTopicDataValue;
use elf_base::{DateTimeUtils, NumericUtils, StdR};
use std::ops::Deref;

impl ArcTopicDataValue {
    /// refer to [is_less_than]
    pub fn is_more_than(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        match self {
            Self::None => {
                if another.is_none_or_empty_str()
                    || another.is_num().is_ok()
                    || another.is_datetime_related()
                {
                    // 1.1, 1.2
                    Ok(false)
                } else {
                    // 1.3
                    self.must_compare_between_num_or_datetime(another)
                }
            }
            Self::Str(one_str) => {
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
            Self::Num(one_decimal) => {
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
            Self::Bool(_) => self.must_compare_between_num_or_datetime(another),
            Self::DateTime(one_datetime) => {
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
            Self::Date(one_date) => {
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
            Self::Time(one_time) => {
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
            Self::Map(_) => self.must_compare_between_num_or_datetime(another),
            // vec is not comparable
            Self::Vec(_) => self.must_compare_between_num_or_datetime(another),
        }
    }
}
