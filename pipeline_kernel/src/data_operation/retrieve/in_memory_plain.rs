use crate::{ArcTopicDataValue, InMemoryData, PipelineKernelErrorCode, PlainDataPath};
use elf_base::{ErrorCode, StdR};
use std::ops::Deref;
use std::sync::Arc;

impl PlainDataPath {
    pub fn value_from_memory(&self, in_memory_data: &InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let prop = self.this_path();
        if in_memory_data.is_current_data_allowed_only() {
            in_memory_data.get_from_current_data(&prop)
        } else {
            in_memory_data.get_from_variables_or_current_data(&prop)
        }
    }

    /// get value from given data by given segment
    /// only map and vec are supported
    /// - when given data is a map, return none when nothing found from this map,
    /// - when given data is a vec, then only none and map element are supported,
    ///   and the returned data is a vec.
    ///   - when given segment is identified as a vec,
    ///     - ignore the none element of given vec,
    ///     - ignore when nothing found from the map element of given vec,
    ///     - ignore the none value found from the map element of given vec,
    pub fn value_from_source(
        &self,
        source: &Arc<ArcTopicDataValue>,
        _in_memory_data: &InMemoryData,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let prop = self.this_path();
        let is_vec = self.is_vec().unwrap_or(false);

        match source.deref() {
            ArcTopicDataValue::Map(map) => {
                if let Some(value) = map.get(&prop) {
                    Ok(value.clone())
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            ArcTopicDataValue::Vec(vec) => {
                let mut values = vec![];
                for vec_elm in vec.iter() {
                    match vec_elm.deref() {
                        ArcTopicDataValue::None => {
                            if !is_vec {
                                values.push(vec_elm.clone());
                            }
                        }
                        ArcTopicDataValue::Map(map) => {
                            if let Some(value) = map.get(&prop) {
                                match value.deref() {
                                    ArcTopicDataValue::None => {
                                        if !is_vec {
                                            values.push(value.clone())
                                        }
                                    }
                                    ArcTopicDataValue::Vec(vec) => {
                                        // flatten
                                        vec.iter().for_each(|value| values.push(value.clone()))
                                    }
                                    _ => values.push(value.clone()),
                                }
                            } else if !is_vec {
                                // when value type is not array, insert a none value
                                values.push(Arc::new(ArcTopicDataValue::None))
                            }
                        }
                        _ => {
                            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                                "Cannot retrieve[path={}, current={}] from parent [{}], caused by element type of vec is not none or map.",
                                self.full_path(), prop, &source
                            ));
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(values))))
            }
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot retrieve[path={}, current={}] from parent [{}], caused by data type is not vec or map.",
                self.full_path(), prop, &source
            )),
        }
    }
}
