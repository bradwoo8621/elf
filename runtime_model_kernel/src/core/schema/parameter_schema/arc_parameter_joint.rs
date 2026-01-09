use crate::{ArcHelper, ArcParameterCondition, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{ParameterJoint, ParameterJointType};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcParameterJoint {
    pub joint_type: Arc<ParameterJointType>,
    pub filters: Arc<Vec<Arc<ArcParameterCondition>>>,
}

impl ArcHelper for ArcParameterJoint {}

impl ArcParameterJoint {
    pub fn new(joint: ParameterJoint) -> StdR<Arc<Self>> {
        let joint_type = Self::must(joint.joint_type, || {
            RuntimeModelKernelErrorCode::ParameterJointTypeMissed
                .msg("Parameter joint must have filter.")
        })?;
        let arc_filters = Self::must_vec(joint.filters, ArcParameterCondition::new, || {
            RuntimeModelKernelErrorCode::ParameterJointFilterMissed
                .msg("Parameter joint must have filter.")
        })?;

        Ok(Arc::new(Self {
            joint_type,
            filters: arc_filters,
        }))
    }
}
