use crate::{PipelineExecuteTopicData, PipelineExecutionVariables};
use elf_auth::Principal;
use elf_model::PipelineTriggerTraceId;
use std::sync::Arc;

/// for execute single pipeline.
/// and it is also the context for executing the pipeline
/// values created during the pipeline execution are saved into variables
pub struct PipelineExecutable {
    pub variables: PipelineExecutionVariables,

    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
}

impl PipelineExecutable {
    pub fn new(
        topic_data: Arc<PipelineExecuteTopicData>,
        principal: Arc<Principal>,
        trace_id: Arc<PipelineTriggerTraceId>,
    ) -> Self {
        PipelineExecutable {
            variables: PipelineExecutionVariables::new(
                topic_data.previous_data().clone(),
                topic_data.current_data().clone(),
            ),
            principal,
            trace_id,
        }
    }
}
