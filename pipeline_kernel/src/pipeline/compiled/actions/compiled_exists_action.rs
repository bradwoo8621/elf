use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledParameterJoint, DataPath,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcExistsAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledExistsAction {
    variable_path: DataPath,
    source_topic_schema: Arc<TopicSchema>,
    source_criteria: CompiledParameterJoint,
}

impl ActionCompiler for CompiledExistsAction {
    type SourceAction = ArcExistsAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcExistsAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let variable_path = ActionCompilerHelper::get_variable_name(
            action.variable_name.as_str(),
            action.action_id.deref(),
            action.r#type.deref(),
        )?;
        let source_topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let source_criteria =
            CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;

        Ok(Self {
            variable_path,
            source_topic_schema,
            source_criteria,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::Exists(compiled_action)
    }
}
