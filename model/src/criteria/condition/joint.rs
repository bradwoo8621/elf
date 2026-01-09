use crate::{BaseDataModel, ModelErrorCode, ParameterCondition, Storable};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, Debug, StrEnum)]
pub enum ParameterJointType {
    And,
    Or,
}

#[adapt_model(storable)]
pub struct ParameterJoint {
    pub joint_type: Option<ParameterJointType>,
    pub filters: Option<Vec<ParameterCondition>>,
}

impl ParameterJoint {
    pub fn and(filters: Vec<ParameterCondition>) -> Self {
        Self {
            joint_type: Some(ParameterJointType::And),
            filters: Some(filters),
        }
    }

    pub fn or(filters: Vec<ParameterCondition>) -> Self {
        Self {
            joint_type: Some(ParameterJointType::Or),
            filters: Some(filters),
        }
    }

    pub fn to_condition(self) -> ParameterCondition {
        ParameterCondition::Joint(self)
    }
}
