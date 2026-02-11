use crate::{ActionCompiler, CompiledAction};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcInsertOrMergeRowAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledInsertOrMergeRowAction;

impl ActionCompiler for CompiledInsertOrMergeRowAction {
    type SourceAction = ArcInsertOrMergeRowAction;
    
    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcInsertOrMergeRowAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledInsertOrMergeRowAction")
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::InsertOrMergeRow(compiled_action)
    }
}
