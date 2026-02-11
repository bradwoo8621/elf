use crate::{ActionCompiler, CompiledAction};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcDeleteRowsAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledDeleteRowsAction;

impl ActionCompiler for CompiledDeleteRowsAction {
    type SourceAction = ArcDeleteRowsAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcDeleteRowsAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledDeleteRowsAction")
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::DeleteRows(compiled_action)
    }
}
