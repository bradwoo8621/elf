use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledMappingFactor,
    CompiledParameterJoint,
};
use elf_base::StdR;
use elf_model::{AccumulateMode, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcPipeline, ArcPipelineStage, ArcPipelineUnit, ArcWriteFactorAction, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledWriteFactorAction {
    target_topic_schema: Arc<TopicSchema>,
    factor_mapping: Vec<CompiledMappingFactor>,
    target_criteria: CompiledParameterJoint,
    accumulate_mode: AccumulateMode,
}

impl ActionCompiler for CompiledWriteFactorAction {
    type SourceAction = ArcWriteFactorAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Arc<ArcWriteFactorAction>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let target_topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let factor_mapping = CompiledMappingFactor::one(
            &target_topic_schema,
            &action.source,
            &action.factor_id,
            &action.arithmetic,
            topic_schemas,
            tenant_id,
        )?;
        let factor_mapping = vec![factor_mapping];
        let target_criteria =
            CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;
        let accumulate_mode = ActionCompilerHelper::unwrap_accumulate_mode(&action.accumulate_mode);

        Ok(Self {
            target_topic_schema,
            factor_mapping,
            target_criteria,
            accumulate_mode,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::WriteFactor(Arc::new(compiled_action))
    }
}
