use crate::{ArcTopicData, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::TopicData;
use std::collections::HashMap;

pub struct PipelineExecutionVariables {
    pub previous_data: Option<ArcTopicData>,
    pub current_data: Option<ArcTopicData>,
    pub variables: TopicData,
    // only variables from trigger data will record its factor name here
    // key is variable key, value is factor name
    pub variables_from: HashMap<String, String>,
}

impl PipelineExecutionVariables {
    pub fn new(previous: Option<ArcTopicData>, current: Option<ArcTopicData>) -> Self {
        PipelineExecutionVariables {
            previous_data: previous,
            current_data: current,
            variables: HashMap::new(),
            variables_from: HashMap::new(),
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
}
