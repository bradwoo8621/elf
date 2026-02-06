use elf_auth::Principal;
use elf_model::{PipelineTriggerTraceId, PipelineTriggerType, TopicData};
use elf_runtime_model_kernel::TopicSchema;
use std::sync::Arc;

pub struct PipelineExecuteInput {
    principal: Arc<Principal>,
    topic_schema: Arc<TopicSchema>,
    trigger_type: Arc<PipelineTriggerType>,
    topic_data: TopicData,
    trace_id: Arc<PipelineTriggerTraceId>,
}

impl PipelineExecuteInput {
    pub fn new(
        principal: Arc<Principal>,
        topic_schema: Arc<TopicSchema>,
        trigger_type: Arc<PipelineTriggerType>,
        topic_data: TopicData,
        trace_id: Arc<PipelineTriggerTraceId>,
    ) -> Self {
        Self {
            principal,
            topic_schema,
            trigger_type,
            topic_data,
            trace_id,
        }
    }
}

impl PipelineExecuteInput {
    pub fn principal(&self) -> Arc<Principal> {
        self.principal.clone()
    }

    pub fn topic_schema(&self) -> Arc<TopicSchema> {
        self.topic_schema.clone()
    }

    pub fn trigger_type(&self) -> Arc<PipelineTriggerType> {
        self.trigger_type.clone()
    }

    pub fn topic_data(self) -> TopicData {
        self.topic_data
    }

    pub fn trace_id(&self) -> Arc<PipelineTriggerTraceId> {
        self.trace_id.clone()
    }
}
