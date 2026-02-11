use crate::{ActionCompiler, CompiledAction, CompiledConditional, DataPath};
use elf_base::StdR;
use elf_model::{AlarmActionSeverity, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcAlarmAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, TopicSchema,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledAlarmAction {
    conditional: CompiledConditional,
    severity: Arc<AlarmActionSeverity>,
    message: Option<DataPath>,
}

impl ActionCompiler for CompiledAlarmAction {
    type SourceAction = ArcAlarmAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcAlarmAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let compiled_conditional =
            CompiledConditional::compile(&action.on, topic_schemas, tenant_id)?;

        Ok(Self {
            conditional: compiled_conditional,
            severity: action.severity.clone(),
            message: if let Some(message) = &action.message {
                Some(DataPath::from_str(message.deref())?)
            } else {
                None
            },
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::Alarm(compiled_action)
    }
}
