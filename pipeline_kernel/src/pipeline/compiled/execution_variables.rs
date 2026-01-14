use crate::{ArcTopicData, ArcTopicDataValue, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PipelineExecutionVariables {
    previous_data: Option<ArcTopicData>,
    current_data: Option<ArcTopicData>,
    variables: HashMap<String, Arc<ArcTopicDataValue>>,
}

impl PipelineExecutionVariables {
    pub fn new(previous: Option<ArcTopicData>, current: Option<ArcTopicData>) -> Self {
        PipelineExecutionVariables {
            previous_data: previous,
            current_data: current,
            variables: HashMap::new(),
        }
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

    /// get previous topic data.
    /// raise error when previous data not exists
    pub fn get_previous_data(&self) -> StdR<&ArcTopicData> {
        match &self.previous_data {
            Some(current_data) => Ok(current_data),
            _ => PipelineKernelErrorCode::PreviousTopicDataMissed
                .msg("Previous of current trigger data is missed."),
        }
    }

    pub fn get_variables(&self) -> &HashMap<String, Arc<ArcTopicDataValue>> {
        &self.variables
    }
}
