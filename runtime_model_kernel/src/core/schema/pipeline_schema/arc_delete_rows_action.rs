use crate::{ArcHelper, ArcParameterJoint};
use std::sync::Arc;
use watchmen_model::{DeleteRowsAction, PipelineActionId, PipelineActionType, StdR, TopicId};

#[derive(Debug)]
pub struct ArcDeleteRowsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// delete from topic
    pub topic_id: Arc<TopicId>,
    /// delete criteria
    pub by: Arc<ArcParameterJoint>,
}

impl ArcHelper for ArcDeleteRowsAction {}

impl ArcDeleteRowsAction {
    pub fn new(action: DeleteRowsAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Delete rows action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Delete rows action[{}]", action_id))?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::DeleteRows),
            topic_id,
            by,
        })
    }
}
