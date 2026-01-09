use crate::{PipelineExecutionLogMonitor, PipelineExecutionVariables, TopicTrigger};
use elf_auth::Principal;
use elf_model::PipelineTriggerTraceId;
use std::sync::Arc;

pub struct PipelineExecutable {
    pub variables: PipelineExecutionVariables,
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub log_monitor: Arc<PipelineExecutionLogMonitor>,
}

impl PipelineExecutable {
    pub fn new(
        topic_trigger: Arc<TopicTrigger>,
        principal: Arc<Principal>,
        trace_id: Arc<PipelineTriggerTraceId>,
        log_monitor: Arc<PipelineExecutionLogMonitor>,
    ) -> Self {
        PipelineExecutable {
            variables: PipelineExecutionVariables::new(
                topic_trigger.previous.clone(),
                topic_trigger.current.clone(),
            ),
            principal,
            trace_id,
            log_monitor,
        }
    }
}
