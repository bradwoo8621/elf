use crate::{ArcHelper, ArcMappingFactor, ArcParameterJoint, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    AccumulateMode, MergeRowAction, PipelineActionId, PipelineActionType, StdErrorCode, StdR,
    TopicId,
};

// noinspection DuplicatedCode
#[derive(Debug)]
pub struct ArcMergeRowAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub accumulate_mode: Arc<AccumulateMode>,
    pub mapping: Arc<Vec<Arc<ArcMappingFactor>>>,
    /// write to topic
    pub topic_id: Arc<TopicId>,
    /// write criteria
    pub by: Arc<ArcParameterJoint>,
}

impl ArcHelper for ArcMergeRowAction {}

impl ArcMergeRowAction {
    pub fn new(action: MergeRowAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let mapping = Self::must_vec(action.mapping, ArcMappingFactor::new, || {
            RuntimeModelKernelErrorCode::ActionMappingFactorMissed.msg(format!(
                "Merge row action[{}] must have mapping.",
                action_id
            ))
        })?;
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Merge row action[{}]", action_id)
        })?;
        let by = Self::action_by(action.by, || format!("Merge row action[{}]", action_id))?;

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::MergeRow),
            accumulate_mode: Arc::new(action.accumulate_mode.unwrap_or(AccumulateMode::Standard)),
            mapping,
            topic_id,
            by,
        })
    }
}
