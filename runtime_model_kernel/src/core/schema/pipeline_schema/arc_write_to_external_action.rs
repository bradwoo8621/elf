use crate::{ArcHelper, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{ExternalWriterId, PipelineActionId, PipelineActionType, WriteToExternalAction};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcWriteToExternalAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub external_writer_id: Arc<ExternalWriterId>,
    pub event_code: Arc<String>,
}

impl ArcHelper for ArcWriteToExternalAction {}

impl ArcWriteToExternalAction {
    pub fn new(action: WriteToExternalAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let external_writer_id = Self::not_blank(
            action.external_writer_id,
            || {
                RuntimeModelKernelErrorCode::ActionExternalWriterIdMissed.msg(format!(
                    "Write to external action[{}] must have an external writer id.",
                    action_id
                ))
            },
            || {
                RuntimeModelKernelErrorCode::ActionExternalWriterIdIsBlank.msg(format!(
                    "Write to external action[{}]'s external writer id cannot be blank.",
                    action_id
                ))
            },
        )?;
        let event_code = Self::not_blank(
            action.event_code,
            || {
                RuntimeModelKernelErrorCode::ActionEventCodeMissed.msg(format!(
                    "Write to external action[{}] must have an event code.",
                    action_id
                ))
            },
            || {
                RuntimeModelKernelErrorCode::ActionEventCodeIsBlank.msg(format!(
                    "Write to external action[{}]'s event code cannot be blank.",
                    action_id
                ))
            },
        )?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::WriteToExternal),
            external_writer_id,
            event_code,
        })
    }
}
