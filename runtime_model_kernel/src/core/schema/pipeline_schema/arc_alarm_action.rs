use crate::{ArcHelper, ArcParameterJoint};
use elf_base::StdR;
use elf_model::{AlarmAction, AlarmActionSeverity, PipelineActionId, PipelineActionType};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcAlarmAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub severity: Arc<AlarmActionSeverity>,
    pub message: Option<Arc<String>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcHelper for ArcAlarmAction {}

impl ArcAlarmAction {
    pub fn new(action: AlarmAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let on = Self::conditional(action.conditional, action.on, || {
            format!(
                "Alarm action[{}] must have condition when conditional is true.",
                action_id
            )
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::Alarm),
            severity: Arc::new(action.severity.unwrap_or(AlarmActionSeverity::Medium)),
            message: action.message.map(Arc::new),
            conditional: on.is_some(),
            on,
        })
    }
}
