use crate::CompiledPipeline;
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};
use watchmen_runtime_model_kernel::{PipelineSchema, TopicSchema};

pub struct PipelineCompileService {
    tenant_id: TenantId,
}

impl PipelineCompileService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {
            tenant_id: tenant_id.clone(),
        }))
    }

    pub fn compile(
        &self,
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
    ) -> StdR<Arc<CompiledPipeline>> {
        Ok(Arc::new(CompiledPipeline::compile(
            topic_schema,
            pipeline_schema,
        )?))
    }
}
