use crate::{InternalPipelineExecutable, PipelineExecutionTask};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcPipelineStage;
use std::sync::Arc;

pub struct CompiledStage {}

impl CompiledStage {
    pub fn compile(stage: &Arc<ArcPipelineStage>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        todo!()
    }
}

impl CompiledStage {
    pub async fn execute(
        &self,
        executable: &InternalPipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        todo!()
    }
}
