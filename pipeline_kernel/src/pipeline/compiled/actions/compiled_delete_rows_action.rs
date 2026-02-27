use crate::{
    generate_compiled_action, ActionCompiler, ActionCompilerHelper, CompiledAction,
    CompiledParameterJoint,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcDeleteRowsAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

generate_compiled_action!(DeleteRows {
    target_topic_schema: Arc<TopicSchema>,
    target_criteria: CompiledParameterJoint,
});

impl ActionCompiler for CompiledDeleteRowsAction {
    type SourceAction = ArcDeleteRowsAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Arc<ArcDeleteRowsAction>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let target_topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let target_criteria =
            CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;

        Ok(Self {
            pipeline: pipeline.clone(),
            stage: stage.clone(),
            unit: unit.clone(),
            action: action.clone(),

            target_topic_schema,
            target_criteria,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::DeleteRows(Arc::new(compiled_action))
    }
}
