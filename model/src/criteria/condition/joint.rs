use crate::{BaseDataModel, ParameterCondition, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
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
