use crate::{ArcHelper, ArcParameterJoint};
use elf_base::StdR;
use elf_model::{
    AggregateArithmetic, FactorId, PipelineActionId, PipelineActionType, ReadFactorAction, TopicId,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcReadFactorAction {
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
    pub arithmetic: Arc<AggregateArithmetic>,
}

impl ArcHelper for ArcReadFactorAction {}

impl ArcReadFactorAction {
    pub fn new(action: ReadFactorAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Read factor action[{}]", action_id)
        })?;
        let factor_id = Self::factor_id(action.factor_id, || {
            format!("Read factor action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Read factor action[{}]", action_id))?;
        let variable_name = Self::variable_name(action.variable_name, || {
            format!("Read factor action[{}]", action_id)
        })?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::ReadFactor),
            topic_id,
            factor_id,
            by,
            variable_name,
            arithmetic: Arc::new(action.arithmetic.unwrap_or(AggregateArithmetic::None)),
        })
    }
}
