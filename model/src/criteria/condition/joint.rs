use crate::{BaseDataModel, ParameterCondition};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ParameterJointType {
    And,
    Or,
}

#[adapt_model(bdm)]
pub struct ParameterJoint {
    pub joint_type: ParameterJointType,
    pub filters: Option<Vec<ParameterCondition>>,
}
