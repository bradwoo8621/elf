use crate::{ArcTopicData, ArcTopicDataValue, DataPath, DataPathSegment, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use std::collections::HashMap;
use std::sync::Arc;

pub struct InMemoryData {
    previous_data: Option<ArcTopicData>,
    current_data: Option<ArcTopicData>,
    variables: HashMap<String, Arc<ArcTopicDataValue>>,

    current_data_only: bool,
}

impl InMemoryData {
    pub fn new(previous_data: Option<ArcTopicData>, current_data: Option<ArcTopicData>) -> Self {
        Self {
            previous_data,
            current_data,
            variables: HashMap::new(),
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

    pub fn is_current_data_allowed_only(&self) -> bool {
        self.current_data_only
    }

    /// get current topic data.
    /// raise error when current data not exists
    pub fn get_current_data(&self) -> StdR<&ArcTopicData> {
        match &self.current_data {
            Some(current_data) => Ok(current_data),
            _ => PipelineKernelErrorCode::CurrentTopicDataMissed
                .msg("Current trigger data is missed."),
        }
    }

    pub fn get_current_data_opt(&self) -> &Option<ArcTopicData> {
        &self.current_data
    }

    /// get previous topic data.
    /// raise error when previous data not exists
    pub fn get_previous_data(&self) -> StdR<&ArcTopicData> {
        match &self.previous_data {
            Some(current_data) => Ok(current_data),
            _ => PipelineKernelErrorCode::PreviousTopicDataMissed
                .msg("Previous of current trigger data is missed."),
        }
    }

    pub fn get_previous_data_opt(&self) -> &Option<ArcTopicData> {
        &self.previous_data
    }

    pub fn get_variables(&self) -> &HashMap<String, Arc<ArcTopicDataValue>> {
        &self.variables
    }
}

impl InMemoryData {
    /// get value from current data by given property.
    pub fn get_from_current_data(&self, prop: &String) -> StdR<Arc<ArcTopicDataValue>> {
        let current_data = self.get_current_data()?;
        let value = current_data.get(prop);
        if let Some(value) = value {
            Ok(value.clone())
        } else {
            Ok(Arc::new(ArcTopicDataValue::None))
        }
    }

    /// get from variables or current data by given property
    pub fn get_from_variables_or_current_data(
        &self,
        prop: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let variables = self.get_variables();
        let value = variables.get(prop);
        if let Some(value) = value {
            Ok(value.clone())
        } else {
            self.get_from_current_data(&prop)
        }
    }

    fn segment_value_from_memory(&self, segment: &DataPathSegment) -> StdR<Arc<ArcTopicDataValue>> {
        match segment {
            DataPathSegment::Plain(plain_path) => plain_path.value_from_memory(&self),
            DataPathSegment::Func(func_path) => func_path.value_from_memory(&self),
        }
    }

    fn segment_value_from_source(
        &self,
        source: &Arc<ArcTopicDataValue>,
        segment: &DataPathSegment,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match segment {
            DataPathSegment::Plain(plain_path) => plain_path.value_from_source(source, &self),
            DataPathSegment::Func(func_path) => func_path.value_from_source(source, &self),
        }
    }

    pub fn value_of(&self, path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let segments = path.segments();
        if let Some((first, rest)) = segments.split_first() {
            let top_value = self.segment_value_from_memory(first)?;
            if rest.is_empty() {
                Ok(top_value)
            } else {
                let mut last_value = top_value;
                for segment in rest {
                    let value = self.segment_value_from_source(&last_value, segment)?;
                    last_value = value;
                }

                Ok(last_value)
            }
        } else {
            PipelineKernelErrorCode::IncorrectDataPath.msg("Data path is empty.")
        }
    }
}
