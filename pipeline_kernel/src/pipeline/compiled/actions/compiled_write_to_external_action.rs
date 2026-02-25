use crate::{ActionCompiler, CompiledAction};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcWriteToExternalAction, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledWriteToExternalAction;

impl ActionCompiler for CompiledWriteToExternalAction {
    type SourceAction = ArcWriteToExternalAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcWriteToExternalAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        todo!("implement compile for CompiledWriteToExternalAction")
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::WriteToExternal(Arc::new(compiled_action))
    }
}
