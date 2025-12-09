use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{DateTimeUtils, NumericUtils, StdR};

trait MinmaxComparator<V: PartialOrd> {
    fn replace_self_if_greater_than(&mut self, b: &Arc<V>);
}

impl<V: PartialOrd> MinmaxComparator<V> for Option<Arc<V>> {
    /// returns min value between self and another
    fn replace_self_if_greater_than(&mut self, another: &Arc<V>) {
        if let Some(one) = self {
            if one.deref() < another {
            } else {
                *self = Some(another.clone())
            }
        } else {
            *self = Some(another.clone())
        }
    }
}

#[derive(Debug)]
struct Minmax {
    has_decimal: bool,
    min_decimal: Option<Arc<BigDecimal>>,
    has_datetime: bool,
    min_datetime: Option<Arc<NaiveDateTime>>,
    has_date: bool,
    min_date: Option<Arc<NaiveDate>>,
    has_time: bool,
    min_time: Option<Arc<NaiveTime>>,

    string_elements: Vec<Arc<String>>,
}

impl Minmax {
    fn new() -> Self {
        Minmax {
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

    /// false means not supported detected
    fn with_decimal(&mut self, decimal: &Arc<BigDecimal>) -> bool {
        if self.has_datetime || self.has_date || self.has_time {
            false
        } else {
            self.has_decimal = true;
            self.min_decimal.replace_self_if_greater_than(decimal);
            true
        }
    }

    /// false means not supported detected
    fn with_datetime(&mut self, datetime: &Arc<NaiveDateTime>) -> bool {
        if self.has_decimal || self.has_time {
            false
        } else {
            self.has_datetime = true;
            self.min_datetime.replace_self_if_greater_than(datetime);
            // datetime also can be compared with date
            self.min_date
                .replace_self_if_greater_than(&Arc::new(datetime.date()));
            true
        }
    }

    /// false means not supported detected
    fn with_date(&mut self, date: &Arc<NaiveDate>) -> bool {
        if self.has_decimal || self.has_time {
            false
        } else {
            self.has_date = true;
            self.min_date.replace_self_if_greater_than(date);
            true
        }
    }

    /// false means not supported detected
    fn with_time(&mut self, time: &Arc<NaiveTime>) -> bool {
        if self.has_decimal || self.has_datetime || self.has_date {
            false
        } else {
            self.has_time = true;
            self.min_time.replace_self_if_greater_than(time);
            true
        }
    }

    /// false means not supported detected
    fn with_decimal_by_string_elements(&mut self) -> bool {
        if self.has_decimal {
            for value in self.string_elements.clone().iter() {
                if let Ok(decimal) = value.to_decimal() {
                    self.with_decimal(&Arc::new(decimal));
                } else {
                    return false;
                }
            }
        }
        true
    }

    /// false means not supported detected
    fn with_date_by_string_elements(&mut self) -> bool {
        if self.has_date {
            for value in self.string_elements.clone().iter() {
                if let Ok(date) = value.to_date_loose() {
                    self.with_date(&Arc::new(date));
                } else {
                    return false;
                }
            }
        }
        true
    }

    /// false means not supported detected
    fn handle_string_elements(&mut self) -> bool {
        if self.string_elements.len() == 0 {
            return true;
        }
        // let _ = self.with_decimal_by_string_elements() && self.with_date_by_string_elements();
        // } else if has_datetime {
        //     let mut downgrade_to_date = false;
        //     // There will still be strings that can only be parsed as the date type and cannot be parsed as datetime.
        //     // When this situation occurs, it will be downgraded to finding the minimum value of the dates.
        //     for value in string_values.iter() {
        //         if !downgrade_to_date && let Ok(datetime) = value.to_datetime() {
        //             min_datetime = datetime.min_of(min_datetime);
        //         } else if let Ok(date) = value.to_date() {
        //             if !downgrade_to_date {
        //                 downgrade_to_date = true;
        //                 min_date = date.min_of(min_datetime);
        //             } else {
        //                 min_date = date.min_of(min_date);
        //             }
        //         } else {
        //             return f1();
        //         }
        //     }
        // } else if has_time {
        //     for value in string_values.iter() {
        //         if let Ok(time) = value.to_time() {
        //             min_time = time.min_of(min_time);
        //         } else {
        //             return f1();
        //         }
        //     }
        // } else {
        //     // TODO no decimal/datetime/date/time, tricky thing!
        //     return not_support();
        // }
        true
    }

    fn try_get_min_value<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
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
            not_support()
        }
    }
}

pub trait ArcTopicDataValueMinmax {
    fn min_of<NotSupport>(
        vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // functions not supported
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        if vec.len() == 0 {
            return Ok(Arc::new(ArcTopicDataValue::None));
        }

        let mut minmax = Minmax::new();

        for value in vec.iter() {
            match value.deref() {
                ArcTopicDataValue::Str(str) => minmax.string_elements.push(str.clone()),
                ArcTopicDataValue::Num(decimal) => {
                    if minmax.with_decimal(decimal) {
                        return not_support();
                    }
                }
                ArcTopicDataValue::DateTime(datetime) => {
                    if minmax.with_datetime(datetime) {
                        return not_support();
                    }
                }
                ArcTopicDataValue::Date(date) => {
                    if minmax.with_date(date) {
                        return not_support();
                    }
                }
                ArcTopicDataValue::Time(time) => {
                    if minmax.with_time(time) {
                        return not_support();
                    }
                }
                ArcTopicDataValue::None => {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                }
                _ => return not_support(),
            }
        }

        minmax.handle_string_elements();
        minmax.try_get_min_value(not_support)
    }
}

impl ArcTopicDataValueMinmax for ArcTopicDataValue {}

#[cfg(test)]
mod tests {
    use crate::topic::arc_topic_data_minmax::Minmax;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn test() {
        let mut minmax = Minmax::new();
        minmax.with_decimal(&Arc::new(BigDecimal::from_str("100").unwrap()));
        minmax.with_decimal(&Arc::new(BigDecimal::from_str("200").unwrap()));
        println!("{:?}", minmax);
    }
}
