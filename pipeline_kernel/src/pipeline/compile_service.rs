use crate::CompiledPipeline;
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};
use watchmen_runtime_model_kernel::PipelineSchema;

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

    pub fn compile(&self, schema: Arc<PipelineSchema>) -> StdR<Arc<CompiledPipeline>> {
        Ok(Arc::new(CompiledPipeline::compile(schema)?))
    }
}
