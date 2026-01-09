use crate::{PipelineExecutionLogMonitor, TopicTrigger};
use elf_auth::Principal;
use elf_model::PipelineTriggerTraceId;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

pub struct PipelineExecution {
    pub topic_schema: Arc<TopicSchema>,
    pub pipeline_schema: Arc<PipelineSchema>,
    pub topic_trigger: Arc<TopicTrigger>,
    // env
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: Arc<PipelineExecutionLogMonitor>,
}
