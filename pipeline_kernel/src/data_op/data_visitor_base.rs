use crate::{
    ArcTopicData, ArcTopicDataValue, DataPath, DataPathSegment, PipelineKernelErrorCode,
    PlainDataPath, VariablePredefineFunctionCaller,
};
use elf_base::{ErrorCode, StdR};
use std::ops::Deref;
use std::sync::Arc;

pub trait DataVisitorBase {
    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_path(&self, parsed_path: &DataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitorBase for ArcTopicData {
    /// get value from given data by given segment
    /// only map and vec are supported
    /// - when given data is a map, return none when nothing found from this map,
    /// - when given data is a vec, then only none and map element are supported,
    ///   and the returned data is a vec.
    ///   - when given segment is identified as a vec,
    ///     - ignore the none element of given vec,
    ///     - ignore when nothing found from the map element of given vec,
    ///     - ignore the none value found from the map element of given vec,
    ///
    /// e.g.
    /// 1. given data is a map, as `{a: 1, b: 2, d: none}`,
    ///    - segment is `a` -> `1`
    ///    - segment is `c` -> `none`
    ///    - segment is `d` -> `none`
    /// 2. given data is a vec, as `[none, {a: [1, 2], b: 3, d: none}]`,
    ///    - segment is `a`, [is_vec] is not true -> `[none, 1, 2]`,
    ///    - segment is `a`, [is_vec] is true -> `[1, 2]`,
    ///    - segment is `c`, [is_vec] is not true -> `[none, none]`
    ///    - segment is `c`, [is_vec] is true -> `[]`
    ///    - segment is `d`, [is_vec] is not true -> `[none, none]`
    ///    - segment is `d`, [is_vec] is true -> `[]`
    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let current_path = &segment.path.to_string();
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
                        ArcTopicDataValue::None => {
                            if !current_is_vec {
                                values.push(value.clone());
                            }
                        }
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
                                "Cannot retrieve[key={}, current={}] from [{:?}], caused by element type of vec is not none or map.",
                                full_path, current_path, &self
                            ));
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(values))))
            }
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot retrieve[key={}, current={}] from [{:?}], caused by data type is not vec or map.",
                full_path, current_path, &self
            )),
        }
    }

    fn value_of_path(&self, parsed_path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let path = &parsed_path.path.to_string();
        let mut data = Arc::new(ArcTopicDataValue::Map(self.clone()));
        for segment in &parsed_path.segments {
            let current_is_vec = match segment {
                DataPathSegment::Func(segment) => {
                    data = VariablePredefineFunctionCaller::prepare(&self, path, segment)
                        .value_of(&data)?;
                    // never mind, just keep the value which returned, no need to do post transforming
                    false
                }
                DataPathSegment::Plain(segment) => {
                    data = self.value_of_plain_segment(&data, segment, path)?;
                    segment.is_vec.unwrap_or(false)
                }
            };

            // recheck the data, is there none, empty vec, then there is no need to go deeper.
            // and when current segment says the value should be a vec, convert none to empty vec
            // and return directly
            match data.deref() {
                ArcTopicDataValue::None => {
                    return if current_is_vec {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    };
                }
                ArcTopicDataValue::Vec(vec) => {
                    if vec.is_empty() {
                        return Ok(data.clone());
                    }
                }
                _ => {}
            }
        }

        Ok(data.clone())
    }
}
