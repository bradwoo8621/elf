use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledParameterJoint, DataPath,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcReadRowsAction, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledReadRowsAction {
    variable_path: DataPath,
    topic_schema: Arc<TopicSchema>,
    by: CompiledParameterJoint,
}

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
        let variable_path = ActionCompilerHelper::get_variable_name(
            action.variable_name.as_str(),
            action.action_id.deref(),
            action.r#type.deref(),
        )?;
        let topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let by = CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;

        Ok(Self {
            variable_path,
            topic_schema,
            by,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::ReadRows(compiled_action)
    }
}
