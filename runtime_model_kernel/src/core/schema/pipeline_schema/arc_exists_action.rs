use crate::{ArcHelper, ArcParameterJoint};
use std::sync::Arc;
use watchmen_model::{ExistsAction, PipelineActionId, PipelineActionType, StdR, TopicId};

#[derive(Debug)]
pub struct ArcExistsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// read from topic
    pub topic_id: Arc<TopicId>,
    /// read criteria
    pub by: Arc<ArcParameterJoint>,
    /// copy to memory variable
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcExistsAction {}

impl ArcExistsAction {
    pub fn new(action: ExistsAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || format!("Exists action[{}]", action_id))?;
        let by = Self::action_by(action.by, || format!("Exists action[{}]", action_id))?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Exists action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::Exists),
            topic_id,
            by,
            variable_name,
        })
    }
}
