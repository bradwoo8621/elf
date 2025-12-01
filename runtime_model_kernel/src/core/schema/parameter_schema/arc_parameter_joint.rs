use crate::{ArcHelper, ArcParameterCondition, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{ParameterJoint, ParameterJointType, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcParameterJoint {
    pub joint_type: Arc<ParameterJointType>,
    pub filters: Arc<Vec<Arc<ArcParameterCondition>>>,
}

impl ArcHelper for ArcParameterJoint {}

impl ArcParameterJoint {
    pub fn new(joint: ParameterJoint) -> StdR<Self> {
        let joint_type = Self::must(joint.joint_type, || {
            RuntimeModelKernelErrorCode::ParameterJointTypeMissed
                .msg("Parameter joint must have filter.")
        })?;
        let arc_filters = Self::must_vec(joint.filters, ArcParameterCondition::new, || {
            RuntimeModelKernelErrorCode::ParameterJointFilterMissed
                .msg("Parameter joint must have filter.")
        })?;

        Ok(Self {
            joint_type,
            filters: arc_filters,
        })
    }

    pub fn new_arc(joint: ParameterJoint) -> StdR<Arc<Self>> {
        Ok(Arc::new(Self::new(joint)?))
    }
}
