use std::collections::HashMap;
use std::sync::Arc;
use watchmen_model::TopicData;

pub struct PipelineExecutionVariables {
    pub previous_data: Option<Arc<TopicData>>,
    pub current_data: Option<Arc<TopicData>>,
    pub variables: TopicData,
    // only variables from trigger data will record its factor name here
    // key is variable key, value is factor name
    pub variables_from: HashMap<String, String>,
}

impl PipelineExecutionVariables {
    pub fn new(previous: Option<Arc<TopicData>>, current: Option<Arc<TopicData>>) -> Self {
        PipelineExecutionVariables {
            previous_data: previous,
            current_data: current,
            variables: HashMap::new(),
            variables_from: HashMap::new(),
        }
    }
}
