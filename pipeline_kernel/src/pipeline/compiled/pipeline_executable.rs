use crate::PipelineExecutionLogMonitor;
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{PipelineTriggerTraceId, TopicData, TopicDataId};

pub struct PipelineExecutable(
    pub Arc<TopicDataId>,
    pub Option<Arc<TopicData>>,
    pub Option<Arc<TopicData>>,
    pub Arc<Principal>,
    pub Arc<PipelineTriggerTraceId>,
    pub Arc<PipelineExecutionLogMonitor>,
);
