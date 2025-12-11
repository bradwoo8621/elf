use crate::{
    ArcTopicDataValue, DataPathSegment, ParsedDataPath, PipelineKernelErrorCode, PlainDataPath,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{StdErr, StdErrCode, StdErrorCode, StdR, VariablePredefineFunctions};

pub trait DataVisitorBase {
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

    /// returns empty vec when first segment identify the value is a vec type
    fn none_value_of_first_segment(
        &self,
        data_path: &ParsedDataPath,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if data_path.segments.is_empty() {
            // never happen, at least segments has one element
            // anyway, return none
            Ok(Arc::new(ArcTopicDataValue::None))
        } else {
            match &data_path.segments[0] {
                DataPathSegment::Plain(first_segment) => {
                    if first_segment.is_vec.unwrap_or(false) {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    }
                }
                DataPathSegment::Func(_) => Ok(Arc::new(ArcTopicDataValue::None)),
            }
        }
    }

    fn value_of_simple_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_func(
        &self,
        value: &Arc<ArcTopicDataValue>,
        func: VariablePredefineFunctions,
        path: &String,
        segment: &String,
    ) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_complex_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitorBase for HashMap<String, Arc<ArcTopicDataValue>> {
    /// simple path has only one segment
    fn value_of_simple_path(&self, path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        // use none if name not exists, never mind the array or not
        let value = self.get(&path.path).clone();
        if value.is_some() {
            Ok(value.unwrap().clone())
        } else {
            self.none_value_of_first_segment(path)
        }
    }

    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let current_path = &segment.path;
        let current_is_vec = &segment.is_vec.unwrap_or(false);

        match data.deref() {
            ArcTopicDataValue::Map(map) => {
                if let Some(value) = map.get(current_path) {
                    Ok(value.clone())
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            ArcTopicDataValue::Vec(vec) => {
                let mut values = vec![];
                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => {}
                        ArcTopicDataValue::Map(map) => {
                            if let Some(value) = map.get(current_path) {
                                match value.deref() {
                                    ArcTopicDataValue::None => {
                                        if !current_is_vec {
                                            values.push(value.clone())
                                        }
                                    }
                                    ArcTopicDataValue::Vec(vec) => {
                                        // flatten
                                        vec.iter().for_each(|value| values.push(value.clone()))
                                    }
                                    _ => values.push(value.clone()),
                                }
                            } else if !current_is_vec {
                                // when value type is not array, insert a none value
                                values.push(Arc::new(ArcTopicDataValue::None))
                            }
                        }
                        _ => {
                            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                                "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                                full_path, current_path, &self
                            ));
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(values))))
            }
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                full_path, current_path, &self
            )),
        }
    }

    fn value_of_func(
        &self,
        value: &Arc<ArcTopicDataValue>,
        func: VariablePredefineFunctions,
        path: &String,
        segment: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let decimal_parse_err = || || self.decimal_parse_error(path, segment);
        let not_support = || || self.function_not_supported(path, segment);
        let not_support_e = || || self.err_function_not_supported(path, segment);

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

    fn value_of_complex_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let path = &parsed_path.path;
        let segments = &parsed_path.segments;
        let first_segment = &segments[0];
        match first_segment {
            DataPathSegment::Func(_) => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Data path[{}] is incorrect, first segment cannot be function.",
                path
            )),
            DataPathSegment::Plain(first_segment) => {
                let data = self.get(&first_segment.path);
                // value not exists
                if data.is_none() {
                    return self.none_value_of_first_segment(parsed_path);
                }

                // loop from index 1
                let mut data = data.unwrap().clone();
                let remain_count = segments.len() - 1;
                let mut current_index = 1;
                while current_index <= remain_count {
                    let segment = &segments[current_index];
                    let current_is_vec = match segment {
                        DataPathSegment::Plain(plain_segment) => {
                            data = self.value_of_plain_segment(&data, plain_segment, path)?;
                            plain_segment.is_vec.unwrap_or(false)
                        }
                        DataPathSegment::Func(_func_segment) => {
                            // TODO
                            // never mind, just keep the value which returned
                            // no need to transform
                            false
                        }
                    };

                    match data.deref() {
                        ArcTopicDataValue::None => {
                            // no need to go deeper
                            return if current_is_vec {
                                Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                            } else {
                                Ok(Arc::new(ArcTopicDataValue::None))
                            };
                        }
                        ArcTopicDataValue::Vec(vec) => {
                            // no need to go deeper, return empty vec directly
                            if vec.is_empty() {
                                return Ok(data.clone());
                            }
                        }
                        _ => {}
                    }

                    // next loop
                    current_index = current_index + 1
                }

                // return get value
                Ok(data.clone())
            }
        }
    }
}
