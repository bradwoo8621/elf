use crate::PipelineExecuteTopicData;
use elf_auth::Principal;
use elf_model::PipelineTriggerTraceId;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

pub struct PipelineExecutionTask {
    principal: Arc<Principal>,
    topic_data: Arc<PipelineExecuteTopicData>,
    topic_schema: Arc<TopicSchema>,
    pipeline_schema: Arc<PipelineSchema>,
    trace_id: Arc<PipelineTriggerTraceId>,
    /// identify that the monitor log is saved asynchronized or not
    async_monitor_log: bool,
}

impl PipelineExecutionTask {
    pub fn new(
        principal: Arc<Principal>,
        topic_data: Arc<PipelineExecuteTopicData>,
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
        trace_id: Arc<PipelineTriggerTraceId>,
        async_monitor_log: bool,
    ) -> Self {
        Self {
            principal,
            topic_data,
            topic_schema,
            pipeline_schema,
            trace_id,
            async_monitor_log,
        }
    }
}

impl PipelineExecutionTask {
    pub fn principal(&self) -> Arc<Principal> {
        self.principal.clone()
    }

    pub fn topic_data(&self) -> Arc<PipelineExecuteTopicData> {
        self.topic_data.clone()
    }

    pub fn topic_schema(&self) -> Arc<TopicSchema> {
        self.topic_schema.clone()
    }

    pub fn pipeline_schema(&self) -> Arc<PipelineSchema> {
        self.pipeline_schema.clone()
    }

    pub fn trace_id(&self) -> Arc<PipelineTriggerTraceId> {
        self.trace_id.clone()
    }

    pub fn async_monitor_log(&self) -> bool {
        self.async_monitor_log
    }
}
