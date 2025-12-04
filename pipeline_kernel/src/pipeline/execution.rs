use crate::{PipelineExecutionLogMonitor, TopicTrigger};
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::PipelineTriggerTraceId;
use watchmen_runtime_model_kernel::{PipelineSchema, TopicSchema};

pub struct PipelineExecution {
    pub topic_schema: Arc<TopicSchema>,
    pub pipeline_schema: Arc<PipelineSchema>,
    pub topic_trigger: Arc<TopicTrigger>,
    // env
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: Arc<PipelineExecutionLogMonitor>,
}
