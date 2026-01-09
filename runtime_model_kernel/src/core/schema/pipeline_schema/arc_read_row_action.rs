use crate::{ArcHelper, ArcParameterJoint};
use elf_base::StdR;
use elf_model::{PipelineActionId, PipelineActionType, ReadRowAction, TopicId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcReadRowAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// read from topic
    pub topic_id: Arc<TopicId>,
    /// read criteria
    pub by: Arc<ArcParameterJoint>,
    /// copy to memory variable
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcReadRowAction {}

impl ArcReadRowAction {
    pub fn new(action: ReadRowAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Read row action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Read row action[{}]", action_id))?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Read row action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::ReadRow),
            topic_id,
            by,
            variable_name,
        })
    }
}
