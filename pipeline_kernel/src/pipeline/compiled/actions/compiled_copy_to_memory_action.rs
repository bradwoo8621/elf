use crate::{
    ActionCompiler, ActionCompilerHelper, CompiledAction, CompiledParameter, DataPath
    ,
};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcCopyToMemoryAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit
    , TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledCopyToMemoryAction {
    variable_path: DataPath,
    source: CompiledParameter,
}

impl ActionCompiler for CompiledCopyToMemoryAction {
    type SourceAction = ArcCopyToMemoryAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcCopyToMemoryAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let variable_path = ActionCompilerHelper::get_variable_name(
            action.variable_name.as_str(),
            action.action_id.deref(),
            action.r#type.deref(),
        )?;
        let source = CompiledParameter::compile(&action.source, topic_schemas, tenant_id)?;

        Ok(Self {
            variable_path,
            source,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::CopyToMemory(compiled_action)
    }
}
