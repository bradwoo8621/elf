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
    pipeline: Arc<ArcPipeline>,
    stage: Arc<ArcPipelineStage>,
    unit: Arc<ArcPipelineUnit>,
    action: Arc<ArcAlarmAction>,

    conditional: Option<CompiledConditional>,
    severity: Arc<AlarmActionSeverity>,
    message: Option<DataPath>,
}

impl CompiledAlarmAction {
    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.pipeline
    }

    pub fn stage(&self) -> &Arc<ArcPipelineStage> {
        &self.stage
    }

    pub fn unit(&self) -> &Arc<ArcPipelineUnit> {
        &self.unit
    }

    pub fn action(&self) -> &Arc<ArcAlarmAction> {
        &self.action
    }

    pub fn conditional(&self) -> &Option<CompiledConditional> {
        &self.conditional
    }

    /// return none when there is no message defined
    pub fn defs(
        &self,
    ) -> Option<(
        &Option<CompiledConditional>,
        &AlarmActionSeverity,
        &DataPath,
    )> {
        match self.message {
            Some(ref message) => Some((&self.conditional, self.severity.deref(), message)),
            None => None,
        }
    }
}

impl ActionCompiler for CompiledAlarmAction {
    type SourceAction = ArcAlarmAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Arc<ArcAlarmAction>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let message = if let Some(message) = &action.message {
            Some(DataPath::from_str(message.deref())?)
        } else {
            None
        };

        // parse condition only when message is defined
        let compiled_conditional = if message.is_some() {
            Some(CompiledConditional::compile(
                &action.on,
                topic_schemas,
                tenant_id,
            )?)
        } else {
            None
        };

        Ok(Self {
            pipeline: pipeline.clone(),
            stage: stage.clone(),
            unit: unit.clone(),
            action: action.clone(),

            conditional: compiled_conditional,
            severity: action.severity.clone(),
            message,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::Alarm(Arc::new(compiled_action))
    }
}
