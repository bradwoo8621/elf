use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcPipelineStage;
use std::sync::Arc;

pub struct CompiledStage {}

impl CompiledStage {
    pub fn compile(_stage: &Arc<ArcPipelineStage>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        todo!("implement compile for CompiledStage")
    }
}
