use crate::{ActionCompiler, CompiledAction};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcReadRowsAction, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledReadRowsAction;

impl ActionCompiler for CompiledReadRowsAction {
    type SourceAction = ArcReadRowsAction;
    
    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcReadRowsAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledReadRowsAction")
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::ReadRows(compiled_action)
    }
}
