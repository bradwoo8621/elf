use crate::{ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledParameterJoint};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcDeleteRowsAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledDeleteRowsAction {
    topic_schema: Arc<TopicSchema>,
    by: CompiledParameterJoint,
}

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
        let topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let by = CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;

        Ok(Self { topic_schema, by })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::DeleteRows(compiled_action)
    }
}
