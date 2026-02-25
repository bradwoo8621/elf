use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledMappingFactor,
    CompiledParameterJoint,
};
use elf_base::StdR;
use elf_model::{AccumulateMode, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcInsertOrMergeRowAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledInsertOrMergeRowAction {
    target_topic_schema: Arc<TopicSchema>,
    factor_mapping: Vec<CompiledMappingFactor>,
    target_criteria: CompiledParameterJoint,
    accumulate_mode: AccumulateMode,
}

impl ActionCompiler for CompiledInsertOrMergeRowAction {
    type SourceAction = ArcInsertOrMergeRowAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Arc<ArcInsertOrMergeRowAction>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let target_topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let target_criteria =
            CompiledParameterJoint::compile(&action.by, topic_schemas, tenant_id)?;
        let factor_mapping = CompiledMappingFactor::create(
            &target_topic_schema,
            &action.mapping,
            topic_schemas,
            tenant_id,
        )?;
        let accumulate_mode = match action.accumulate_mode.deref() {
            AccumulateMode::Standard => AccumulateMode::Standard,
            AccumulateMode::Cumulate => AccumulateMode::Cumulate,
            // for insert or merge, reverse is not allowed, use standard instead
            AccumulateMode::Reverse => AccumulateMode::Standard,
        };

        Ok(Self {
            target_topic_schema,
            factor_mapping,
            target_criteria,
            accumulate_mode,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::InsertOrMergeRow(Arc::new(compiled_action))
    }
}
