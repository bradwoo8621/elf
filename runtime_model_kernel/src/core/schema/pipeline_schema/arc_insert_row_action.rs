use crate::{ArcHelper, ArcMappingFactor, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{AccumulateMode, InsertRowAction, PipelineActionId, PipelineActionType, TopicId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcInsertRowAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub accumulate_mode: Arc<AccumulateMode>,
    pub mapping: Arc<Vec<Arc<ArcMappingFactor>>>,
    /// write to topic
    pub topic_id: Arc<TopicId>,
}

impl ArcHelper for ArcInsertRowAction {}

impl ArcInsertRowAction {
    pub fn new(action: InsertRowAction) -> StdR<Arc<Self>> {
        let action_id = Self::or_empty_str(action.action_id);
        let mapping = Self::must_vec(action.mapping, ArcMappingFactor::new, || {
            RuntimeModelKernelErrorCode::ActionMappingFactorMissed.msg(format!(
                "Insert row action[{}] must have mapping.",
                action_id
            ))
        })?;
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Insert row action[{}]", action_id)
        })?;

        Ok(Arc::new(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::InsertRow),
            accumulate_mode: Arc::new(action.accumulate_mode.unwrap_or(AccumulateMode::Standard)),
            mapping,
            topic_id,
        }))
    }
}
