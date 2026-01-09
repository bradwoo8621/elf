use elf_auth::Principal;
use elf_model::PipelineTriggerTraceId;
use std::sync::Arc;

pub struct PipelineExecutionLogMonitor {
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
}
