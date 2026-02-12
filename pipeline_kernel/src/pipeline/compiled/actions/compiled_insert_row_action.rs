use crate::{ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledMappingFactor};
use elf_base::StdR;
use elf_model::{AccumulateMode, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcInsertRowAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledInsertRowAction {
    target_topic_schema: Arc<TopicSchema>,
    factor_mapping: Vec<CompiledMappingFactor>,
    accumulate_mode: AccumulateMode,
}

impl ActionCompiler for CompiledInsertRowAction {
    type SourceAction = ArcInsertRowAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcInsertRowAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let target_topic_schema =
            ActionCompilerHelper::find_topic_schema(&action.topic_id, tenant_id, topic_schemas)?;
        let factor_mapping = CompiledMappingFactor::create(
            &target_topic_schema,
            &action.mapping,
            topic_schemas,
            tenant_id,
        )?;
        // always be standard for insertion
        let accumulate_mode = AccumulateMode::Standard;

        Ok(Self {
            target_topic_schema,
            factor_mapping,
            accumulate_mode,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::InsertRow(compiled_action)
    }
}
