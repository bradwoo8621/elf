use crate::{serde_for_enum, BaseDataModel, ParameterCondition};
use watchmen_model_marco::{adapt_model, Display};

#[derive(Display)]
pub enum ParameterJointType {
    And,
    Or,
}

serde_for_enum! {
    ParameterJointType {
        And => "and",
        Or => "or",
    }
}

#[adapt_model(bdm)]
pub struct ParameterJoint {
    pub joint_type: ParameterJointType,
    pub filters: Option<Vec<ParameterCondition>>,
}
