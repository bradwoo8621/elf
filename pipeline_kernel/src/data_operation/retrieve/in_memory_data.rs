use crate::{
    ArcTopicData, ArcTopicDataValue, DataPath, DataPathSegment, FuncDataPath,
    PipelineExecutionVariables, PipelineKernelErrorCode, PlainDataPath,
};
use elf_base::{ErrorCode, StdR};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct InMemoryData<'a> {
    variables: &'a PipelineExecutionVariables,
    current_data_only: bool,
}

impl<'a> InMemoryData<'a> {
    pub fn new(variables: &'a PipelineExecutionVariables) -> Self {
        Self {
            variables,
            current_data_only: false,
        }
    }

    /// get an instance which allowed get value from current data only
    pub fn current_only(&mut self) -> &Self {
        self.current_data_only = true;
        self
    }

    /// get an instance which allowed get value from anywhere (current, previous and variables)
    pub fn all_allowed(&mut self) -> &Self {
        self.current_data_only = false;
        self
    }

    /// get current topic data.
    /// raise error when current data not exists
    pub fn get_current_data(&self) -> StdR<&ArcTopicData> {
        self.variables.get_current_data()
    }

    /// get previous topic data.
    /// raise error when previous data not exists
    pub fn get_previous_data(&self) -> StdR<&ArcTopicData> {
        self.variables.get_previous_data()
    }

    pub fn get_variables(&self) -> &HashMap<String, Arc<ArcTopicDataValue>> {
        self.variables.get_variables()
    }
}

impl InMemoryData<'_> {
    fn get_from_current_data(&self, prop: &String) -> StdR<Arc<ArcTopicDataValue>> {
        let current_data = self.get_current_data()?;
        let value = current_data.get(prop);
        if let Some(value) = value {
            Ok(value.clone())
        } else {
            Ok(Arc::new(ArcTopicDataValue::None))
        }
    }

    fn get_first_part_by_plain_path(&self, path: &PlainDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let prop = path.this_path();
        if self.current_data_only {
            self.get_from_current_data(&prop)
        } else {
            // get from variables first
            let variables = self.get_variables();
            let value = variables.get(&prop);
            if let Some(value) = value {
                Ok(value.clone())
            } else {
                self.get_from_current_data(&prop)
            }
        }
    }

    fn get_first_part_by_func_path(&self, path: &FuncDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        path.get_value(&self)
    }

    fn get_first_part(&self, segment: &DataPathSegment) -> StdR<Arc<ArcTopicDataValue>> {
        match segment {
            DataPathSegment::Plain(plain_path) => self.get_first_part_by_plain_path(plain_path),
            DataPathSegment::Func(func_path) => self.get_first_part_by_func_path(func_path),
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
    fn get_rest_part_by_plain_path(
        &self,
        parent: &Arc<ArcTopicDataValue>,
        path: &PlainDataPath,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let prop = path.this_path();
        let is_vec = path.is_vec().unwrap_or(false);

        match parent.deref() {
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
                                path.full_path(), prop, &parent
                            ));
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(values))))
            }
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot retrieve[path={}, current={}] from parent [{}], caused by data type is not vec or map.",
                path.full_path(), prop, &parent
            )),
        }
    }

    fn get_rest_part_by_func_path(
        &self,
        parent: &Arc<ArcTopicDataValue>,
        path: &FuncDataPath,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        path.get_value_of(parent, &self)
    }

    fn get_rest_part(
        &self,
        parent: &Arc<ArcTopicDataValue>,
        segment: &DataPathSegment,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match segment {
            DataPathSegment::Plain(plain_path) => {
                self.get_rest_part_by_plain_path(parent, plain_path)
            }
            DataPathSegment::Func(func_path) => self.get_rest_part_by_func_path(parent, func_path),
        }
    }

    pub fn get_value(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let segments = path.segments();
        if let Some((first, rest)) = segments.split_first() {
            let top_value = self.get_first_part(first)?;
            if rest.is_empty() {
                Ok(top_value)
            } else {
                let mut last_value = top_value;
                for segment in rest {
                    let value = self.get_rest_part(&last_value, segment)?;
                    last_value = value;
                }

                Ok(last_value)
            }
        } else {
            PipelineKernelErrorCode::IncorrectDataPath.msg("Data path is empty.")
        }
    }
}
