use crate::{ArcHelper, ArcParameter, ArcParameterJoint};
use std::sync::Arc;
use watchmen_model::{
    AccumulateMode, AggregateArithmetic, FactorId, PipelineActionId,
    PipelineActionType, StdR, TopicId, WriteFactorAction,
};

#[derive(Debug)]
pub struct ArcWriteFactorAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub accumulate_mode: Arc<AccumulateMode>,
    pub source: Arc<ArcParameter>,
    /// write to topic
    pub topic_id: Arc<TopicId>,
    /// write to factor
    pub factor_id: Arc<FactorId>,
    /// write criteria
    pub by: Arc<ArcParameterJoint>,
    pub arithmetic: Arc<AggregateArithmetic>,
}

impl ArcHelper for ArcWriteFactorAction {}

impl ArcWriteFactorAction {
    pub fn new(action: WriteFactorAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let source = Self::action_source(action.source, || {
            format!("Write factor action[{}]", action_id)
        })?;
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Write factor action[{}]", action_id)
        })?;
        let factor_id = Self::factor_id(action.factor_id, || {
            format!("Write factor action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Write factor action[{}]", action_id))?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::WriteFactor),
            accumulate_mode: Arc::new(action.accumulate_mode.unwrap_or(AccumulateMode::Standard)),
            source,
            topic_id,
            factor_id,
            by,
            arithmetic: Arc::new(action.arithmetic.unwrap_or(AggregateArithmetic::None)),
        })
    }
}
