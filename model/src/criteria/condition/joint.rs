use super::ParameterCondition;
use crate::serde_for_enum;
use crate::BaseDataModel;
use std::fmt;

pub enum ParameterJointType {
    And,
    Or,
}

impl fmt::Display for ParameterJointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterJointType::And => write!(f, "and"),
            ParameterJointType::Or => write!(f, "or"),
        }
    }
}

serde_for_enum! {
    ParameterJointType {
        And => "and",
        Or => "or",
    }
}

pub struct ParameterJoint {
    pub joint_type: ParameterJointType,
    pub filters: Option<Vec<ParameterCondition>>,
}

impl BaseDataModel for ParameterJoint {}
