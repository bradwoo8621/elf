use crate::{ArcFrom, ArcTopicDataValue, InMemoryData, PipelineKernelErrorCode, PlainDataPath};
use elf_base::{ErrorCode, StdR};
use std::ops::Deref;
use std::sync::Arc;

impl PlainDataPath {
    pub fn value_from_memory(&self, in_memory_data: &InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let prop = self.this_path();
        let value = if in_memory_data.is_current_data_allowed_only() {
            in_memory_data.get_from_current_data(&prop)?
        } else {
            in_memory_data.get_from_variables_or_current_data(&prop)?
        };
        if self.is_vec().unwrap_or(false) {
            match value.deref() {
                // return empty vec
                ArcTopicDataValue::None => Ok(ArcTopicDataValue::arc_from(vec![])),
                _ => Ok(value),
            }
        } else {
            Ok(value)
        }
    }

    /// get value from given data by given segment
    /// only map and vec are supported
    /// and for vec source and vec value, following rules as below:
    /// - when get value from vec source, if value from element is
    ///   - none or [ArcTopicDataValue::None], ignore
    ///   - or flatten,
    /// - when value is none, is current path is vec, return empty vec
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
