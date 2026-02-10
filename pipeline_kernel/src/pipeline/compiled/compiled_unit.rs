use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledUnit {}

impl CompiledUnit {
    pub fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledUnit")
    }
}
