use elf_base::StdR;
use crate::{ArcTopicDataValue, DataPath, PipelineExecutionVariables};

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
}

impl InMemoryData<'_> {
    pub fn get_value(&self, path: &DataPath) -> StdR<ArcTopicDataValue> {
        // TODO
        Ok(ArcTopicDataValue::None)
    }
}
