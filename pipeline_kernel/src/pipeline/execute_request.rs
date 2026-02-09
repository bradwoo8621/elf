use crate::{PipelineExecuteTopicData, PipelineExecutionContext};
use elf_auth::Principal;
use elf_model::{PipelineTriggerTraceId, TopicDataId};
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

pub struct PipelineExecuteRequest {
    principal: Arc<Principal>,
    topic_data: Arc<PipelineExecuteTopicData>,
    topic_schema: Arc<TopicSchema>,
    pipeline_schemas: Vec<Arc<PipelineSchema>>,
    trace_id: Arc<PipelineTriggerTraceId>,
}

impl PipelineExecuteRequest {
    pub fn create(
        principal: Arc<Principal>,
        topic_data: PipelineExecuteTopicData,
        topic_schema: Arc<TopicSchema>,
        pipeline_schemas: Vec<Arc<PipelineSchema>>,
        trace_id: Arc<PipelineTriggerTraceId>,
    ) -> Self {
        Self {
            principal,
            topic_data: Arc::new(topic_data),
            topic_schema,
            pipeline_schemas,
            trace_id,
        }
    }
}

impl PipelineExecuteRequest {
    pub fn topic_data_id(&self) -> Arc<TopicDataId> {
        self.topic_data.topic_data_id().clone()
    }

    pub fn topic_schema(&self) -> Arc<TopicSchema> {
        self.topic_schema.clone()
    }

    pub fn create_execution_context(self, async_monitor_log: bool) -> PipelineExecutionContext {
        PipelineExecutionContext::create(
            self.principal,
            self.topic_data,
            self.topic_schema,
            self.pipeline_schemas,
            self.trace_id,
            async_monitor_log
        )
    }
}
