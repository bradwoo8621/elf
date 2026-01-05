use crate::{ArcTopicDataValue, Minmax};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use watchmen_base::{StdErr, StdR, StringConverter};

impl ArcTopicDataValue {
    /// try to count, can only apply to vec or map
    /// otherwise raise error by given functions
    pub fn count<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Vec(vec) => BigDecimal::from_usize(vec.len())
                .map(|value| Ok(Arc::new(Self::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            Self::Map(map) => BigDecimal::from_usize(map.len())
                .map(|value| Ok(Arc::new(Self::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            _ => not_support(),
        }
    }

    /// get chars count of string, or decimal to string
    pub fn length<DecimalParseErr, NotSupport>(
        &self,
        decimal_parse_err: DecimalParseErr,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Str(str) => BigDecimal::from_usize(str.chars().count())
                .map(|value| Ok(Arc::new(Self::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            Self::Num(decimal) => {
                BigDecimal::from_usize(String::from_decimal(decimal).chars().count())
                    .map(|value| Ok(Arc::new(Self::Num(Arc::new(value)))))
                    .unwrap_or(decimal_parse_err())
            }
            _ => not_support(),
        }
    }

    /// distinct elements, can be applied on vec only
    /// for each element in vec,
    /// - str, decimal, datetime, date, time -> with the same type and value will be distinct,
    /// - bool -> maximum 2: true and false,
    /// - none -> maximum 1
    /// - vec, map -> cannot be removed as duplicates and are always added to the result.
    pub fn distinct<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Vec(vec) => {
                let mut distinct_values: Vec<Arc<Self>> = vec![];

                let mut none_added = false;
                let mut true_added = false;
                let mut false_added = false;
                let mut string_values = HashMap::new();
                let mut decimal_values = HashMap::new();
                let mut datetime_values = HashMap::new();
                let mut date_values = HashMap::new();
                let mut time_values = HashMap::new();

                vec.iter().for_each(|value| {
                    let should_add = match value.deref() {
                        Self::Str(str) => {
                            if !string_values.contains_key(str) {
                                string_values.insert(str.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Num(decimal) => {
                            if !decimal_values.contains_key(decimal) {
                                decimal_values.insert(decimal.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Bool(bool) => {
                            if *bool && !true_added {
                                true_added = true;
                                true
                            } else if !*bool && !false_added {
                                false_added = true;
                                true
                            } else {
                                false
                            }
                        }
                        Self::DateTime(datetime) => {
                            if !datetime_values.contains_key(datetime) {
                                datetime_values.insert(datetime.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Date(date) => {
                            if !date_values.contains_key(date) {
                                date_values.insert(date.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Time(time) => {
                            if !time_values.contains_key(time) {
                                time_values.insert(time.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Vec(_) => true,
                        Self::Map(_) => true,
                        Self::None => {
                            if !none_added {
                                none_added = true;
                                true
                            } else {
                                false
                            }
                        }
                    };
                    if should_add {
                        distinct_values.push(value.clone());
                    }
                });

                Ok(Arc::new(Self::Vec(Arc::new(distinct_values))))
            }
            _ => not_support(),
        }
    }

    /// 1. return cloned string when self is string
    /// 2. return joined string when self is vec, and element of vec cannot be vec or map. note the none value is ignored
    pub fn join<NotSupport>(
        &self,
        sep: &str,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Str(str) => Ok(Arc::new(Self::Str(str.clone()))),
            Self::Vec(vec) => {
                if vec.len() == 0 {
                    Ok(Arc::new(Self::Str(Arc::new("".to_string()))))
                } else {
                    let mut segments: Vec<String> = vec![];
                    for value in vec.iter() {
                        match value.deref() {
                            Self::Str(str) => {
                                segments.push(str.to_string());
                            }
                            Self::Num(decimal) => {
                                segments.push(String::from_decimal(decimal.deref()));
                            }
                            Self::Bool(bool) => {
                                segments.push(String::from_bool(bool));
                            }
                            Self::DateTime(datetime) => {
                                segments.push(String::from_datetime(datetime));
                            }
                            Self::Date(date) => {
                                segments.push(String::from_date(date));
                            }
                            Self::Time(time) => {
                                segments.push(String::from_time(time));
                            }
                            Self::None => {}
                            _ => return not_support(),
                        }
                    }
                    Ok(Arc::new(Self::Str(Arc::new(segments.join(sep)))))
                }
            }
            _ => not_support(),
        }
    }

    /// get the min value of vec elements, only decimal/datetime/date/time can be compared
    /// - if there is no element in vec, returns none,
    /// - none or empty string ignored,
    /// - all elements must, can be converted to one single type,
    /// - if there are datetime and date, returns date.
    pub fn min<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [min], but only decimal and string
    pub fn min_decimal<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_decimal_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [min], but only date
    pub fn min_date<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_date_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [min], but only datetime and date
    pub fn min_datetime<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_datetime_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [min], but only time
    pub fn min_time<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.min_time_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [min]
    pub fn max<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [max], but only decimal and string
    pub fn max_decimal<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_decimal_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [max], but only date
    pub fn max_date<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_date_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [max], but only datetime and date
    pub fn max_datetime<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_datetime_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// refer to [max], but only time
    pub fn max_time<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => vec.max_time_value(not_support),
            _ => Err(not_support()),
        }
    }

    /// none and empty string are treated as 0
    /// return 0 when there is no elements.
    pub fn sum<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => {
                let mut sum: BigDecimal = BigDecimal::zero();
                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => continue,
                        ArcTopicDataValue::Str(str) => {
                            if !str.is_empty() {
                                let decimal = value.try_to_decimal()?;
                                sum = sum + decimal.deref();
                            }
                        }
                        _ => {
                            let decimal = value.try_to_decimal()?;
                            sum = sum + decimal.deref();
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(sum))))
            }
            _ => not_support(),
        }
    }

    /// none and empty string are treated as 0, not count
    /// return 0 when there is no elements.
    pub fn avg<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(
                        BigDecimal::zero(),
                    ))));
                }

                let mut sum: BigDecimal = BigDecimal::zero();
                let mut count = 0;

                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => continue,
                        ArcTopicDataValue::Str(str) => {
                            if !str.is_empty() {
                                let decimal = value.try_to_decimal()?;
                                sum = sum + decimal.deref();
                                count = count + 1;
                            }
                        }
                        _ => {
                            let decimal = value.try_to_decimal()?;
                            sum = sum + decimal.deref();
                            count = count + 1;
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(sum / count))))
            }
            _ => not_support(),
        }
    }
}
