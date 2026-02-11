use crate::{ActionCompiler, CompiledAction};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcReadRowAction, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledReadRowAction;

impl ActionCompiler for CompiledReadRowAction {
    type SourceAction = ArcReadRowAction;
    
    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcReadRowAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledReadRowAction")
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::ReadRow(compiled_action)
    }
}
