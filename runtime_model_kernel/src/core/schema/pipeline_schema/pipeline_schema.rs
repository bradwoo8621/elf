use crate::ArcPipeline;
use elf_base::StdR;
use elf_model::{Pipeline, PipelineId, PipelineTriggerType, TenantId};
use std::sync::Arc;

pub struct PipelineSchema {
    inner: Arc<ArcPipeline>,
}

impl PipelineSchema {
    pub fn new(pipeline: Pipeline) -> StdR<Self> {
        Ok(PipelineSchema {
            inner: ArcPipeline::new(pipeline)?,
        })
    }

    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.inner
    }

    pub fn pipeline_id(&self) -> &Arc<PipelineId> {
        &self.inner.pipeline_id
    }

    pub fn name(&self) -> &Arc<String> {
        &self.pipeline().name
    }

    pub fn r#type(&self) -> &Arc<PipelineTriggerType> {
        &self.pipeline().r#type
    }

    pub fn tenant_id(&self) -> &Arc<TenantId> {
        &self.inner.tenant_id
    }
}
