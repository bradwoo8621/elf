use crate::{ArcTopicData, ArcTopicDataValue, Minmax, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{
    StdErr, StdErrCode, StdErrorCode, StdR, StringConverter, VariablePredefineFunctions,
};

pub enum TopicDataProperty {
    /// 0. property name,
    /// 1. is array or not
    Str((String, bool)),
    /// 0. property name,
    /// 1. names split by [.],
    /// 2. is array or not
    Vec((String, Vec<String>, bool)),
}

impl TopicDataProperty {
    ///
    pub fn of(property: &String, array: bool) -> Self {
        if property.contains('.') {
            TopicDataProperty::Vec((
                property.clone(),
                property.split('.').map(|s| s.to_string()).collect(),
                array,
            ))
        } else {
            TopicDataProperty::Str((property.clone(), array))
        }
    }
}

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
            ArcTopicDataValue::Vec(vec) => BigDecimal::from_usize(vec.len())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            ArcTopicDataValue::Map(map) => BigDecimal::from_usize(map.len())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
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
            ArcTopicDataValue::Str(str) => BigDecimal::from_usize(str.chars().count())
                .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
                .unwrap_or(decimal_parse_err()),
            ArcTopicDataValue::Num(decimal) => {
                BigDecimal::from_usize(String::from_decimal(decimal).chars().count())
                    .map(|value| Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(value)))))
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
            ArcTopicDataValue::Vec(vec) => {
                let mut distinct_values: Vec<Arc<ArcTopicDataValue>> = vec![];

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
                        ArcTopicDataValue::Str(str) => {
                            if !string_values.contains_key(str) {
                                string_values.insert(str.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Num(decimal) => {
                            if !decimal_values.contains_key(decimal) {
                                decimal_values.insert(decimal.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Bool(bool) => {
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
                        ArcTopicDataValue::DateTime(datetime) => {
                            if !datetime_values.contains_key(datetime) {
                                datetime_values.insert(datetime.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Date(date) => {
                            if !date_values.contains_key(date) {
                                date_values.insert(date.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Time(time) => {
                            if !time_values.contains_key(time) {
                                time_values.insert(time.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        ArcTopicDataValue::Vec(_) => true,
                        ArcTopicDataValue::Map(_) => true,
                        ArcTopicDataValue::None => {
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

                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(distinct_values))))
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
            ArcTopicDataValue::Str(str) => Ok(Arc::new(ArcTopicDataValue::Str(str.clone()))),
            ArcTopicDataValue::Vec(vec) => {
                if vec.len() == 0 {
                    Ok(Arc::new(ArcTopicDataValue::Str(Arc::new("".to_string()))))
                } else {
                    let mut segments: Vec<String> = vec![];
                    for value in vec.iter() {
                        match value.deref() {
                            ArcTopicDataValue::Str(str) => {
                                segments.push(str.to_string());
                            }
                            ArcTopicDataValue::Num(decimal) => {
                                segments.push(String::from_decimal(decimal.deref()));
                            }
                            ArcTopicDataValue::Bool(bool) => {
                                segments.push(String::from_bool(bool));
                            }
                            ArcTopicDataValue::DateTime(datetime) => {
                                segments.push(String::from_datetime(datetime));
                            }
                            ArcTopicDataValue::Date(date) => {
                                segments.push(String::from_date(date));
                            }
                            ArcTopicDataValue::Time(time) => {
                                segments.push(String::from_time(time));
                            }
                            ArcTopicDataValue::None => {}
                            _ => return not_support(),
                        }
                    }
                    Ok(Arc::new(ArcTopicDataValue::Str(Arc::new(
                        segments.join(sep),
                    ))))
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

trait TopicDataUtilsBase {
    fn decimal_parse_error<R>(&self, name: &String, current_name: &String) -> StdR<R>
    where
        Self: Debug,
    {
        StdErrCode::DecimalParse.msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
            name, current_name, &self
        ))
    }

    fn function_not_supported<R>(&self, name: &String, current_name: &String) -> StdR<R>
    where
        Self: Debug,
    {
        Err(self.err_function_not_supported(name, current_name))
    }

    fn err_function_not_supported(&self, name: &String, current_name: &String) -> StdErr
    where
        Self: Debug,
    {
        PipelineKernelErrorCode::VariableFuncNotSupported.e_msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
            name, current_name, &self
        ))
    }
}

impl TopicDataUtilsBase for ArcTopicData {}

pub trait TopicDataUtils {
    fn value_of_func(
        &self,
        value: &Arc<ArcTopicDataValue>,
        func: VariablePredefineFunctions,
        name: &String,
        current_name: &String,
    ) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>>;
}

impl TopicDataUtils for ArcTopicData {
    fn value_of_func(
        &self,
        value: &Arc<ArcTopicDataValue>,
        func: VariablePredefineFunctions,
        name: &String,
        current_name: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let decimal_parse_err = || || self.decimal_parse_error(name, current_name);
        let not_support = || || self.function_not_supported(name, current_name);
        let not_support_e = || || self.err_function_not_supported(name, current_name);

        match func {
            VariablePredefineFunctions::Count => value.count(decimal_parse_err(), not_support()),
            VariablePredefineFunctions::Length | VariablePredefineFunctions::Len => {
                value.length(decimal_parse_err(), not_support())
            }
            VariablePredefineFunctions::Join => value.join(",", not_support()),
            VariablePredefineFunctions::Distinct => value.distinct(not_support()),
            VariablePredefineFunctions::Min => value.min(not_support_e()),
            VariablePredefineFunctions::MinNum => value.min_decimal(not_support_e()),
            VariablePredefineFunctions::MinDate => value.min_date(not_support_e()),
            VariablePredefineFunctions::MinDatetime | VariablePredefineFunctions::MinDt => {
                value.min_datetime(not_support_e())
            }
            VariablePredefineFunctions::MinTime => value.min_time(not_support_e()),
            VariablePredefineFunctions::Max => value.max(not_support_e()),
            VariablePredefineFunctions::MaxNum => value.max_decimal(not_support_e()),
            VariablePredefineFunctions::MaxDate => value.max_date(not_support_e()),
            VariablePredefineFunctions::MaxDatetime | VariablePredefineFunctions::MaxDt => {
                value.max_datetime(not_support_e())
            }
            VariablePredefineFunctions::MaxTime => value.max_time(not_support_e()),
            VariablePredefineFunctions::Sum => value.sum(not_support()),
            VariablePredefineFunctions::Avg => value.avg(not_support()),
            _ => not_support()(),
        }
    }

    fn value_of(&self, property: &TopicDataProperty) -> StdR<Arc<ArcTopicDataValue>> {
        match property {
            TopicDataProperty::Str((name, _)) => {
                // use none if name not exists, never mind the array or not
                let value = self.get(name).clone();
                if value.is_some() {
                    Ok(value.unwrap().clone())
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            TopicDataProperty::Vec((name, names, array)) => {
                let data = self.get(&names[0]);
                if data.is_none() {
                    return if *array {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    };
                }

                let mut data = data.unwrap();
                let mut remain_count = names.len() - 1;
                let mut current_index = 1;
                while current_index <= remain_count {
                    let current_name = &names[current_index];
                    if let Some(func) = VariablePredefineFunctions::try_parse(current_name) {
                        // func exactly matched always is the last part, so return directly
                        return self.value_of_func(data, func, name, current_name);
                    } else {
                    }
                }

                Ok(Arc::new(ArcTopicDataValue::None))
            }
        }
    }
}
