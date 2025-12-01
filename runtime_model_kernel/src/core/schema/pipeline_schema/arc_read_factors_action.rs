use crate::{ArcHelper, ArcParameterJoint};
use std::sync::Arc;
use watchmen_model::{
    FactorId, PipelineActionId, PipelineActionType, ReadFactorsAction, StdR, TopicId,
};

#[derive(Debug)]
pub struct ArcReadFactorsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// read from topic
    pub topic_id: Arc<TopicId>,
    /// read from factor
    pub factor_id: Arc<FactorId>,
    /// read criteria
    pub by: Arc<ArcParameterJoint>,
    /// copy to memory variable
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcReadFactorsAction {}

impl ArcReadFactorsAction {
    pub fn new(action: ReadFactorsAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Read factors action[{}]", action_id)
        })?;
        let factor_id = Self::factor_id(action.factor_id, || {
            format!("Read factors action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Read factors action[{}]", action_id))?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Read factors action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::ReadFactors),
            topic_id,
            factor_id,
            by,
            variable_name,
        })
    }
}
