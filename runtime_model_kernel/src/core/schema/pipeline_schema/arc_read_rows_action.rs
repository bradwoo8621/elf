use crate::{ArcHelper, ArcParameterJoint};
use elf_base::StdR;
use elf_model::{PipelineActionId, PipelineActionType, ReadRowsAction, TopicId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcReadRowsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// read from topic
    pub topic_id: Arc<TopicId>,
    /// read criteria
    pub by: Arc<ArcParameterJoint>,
    /// copy to memory variable
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcReadRowsAction {}

impl ArcReadRowsAction {
    pub fn new(action: ReadRowsAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Read rows action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Read rows action[{}]", action_id))?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Read rows action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::ReadRows),
            topic_id,
            by,
            variable_name,
        })
    }
}
