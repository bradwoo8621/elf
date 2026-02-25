use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledParameterJoint, DataPath,
};
use elf_base::StdR;
use elf_model::{AggregateArithmetic, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcFactor, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcReadFactorAction, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledReadFactorAction {
    variable_path: DataPath,
    source_topic_schema: Arc<TopicSchema>,
    source_factor: Arc<ArcFactor>,
    source_criteria: CompiledParameterJoint,
    aggregate_arithmetic: AggregateArithmetic,
}

impl ActionCompiler for CompiledReadFactorAction {
    type SourceAction = ArcReadFactorAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcReadFactorAction,
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
        let source_factor =
            ActionCompilerHelper::find_factor(source_topic_schema.deref(), &action.factor_id)?;
        let source_criteria =
            CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;
        let aggregate_arithmetic =
            ActionCompilerHelper::unwrap_aggregate_arithmetic(&action.arithmetic);

        Ok(Self {
            variable_path,
            source_topic_schema,
            source_factor,
            source_criteria,
            aggregate_arithmetic,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::ReadFactor(Arc::new(compiled_action))
    }
}
