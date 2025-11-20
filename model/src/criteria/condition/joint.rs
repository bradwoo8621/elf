use crate::{BaseDataModel, ParameterCondition, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ParameterJointType {
    And,
    Or,
}

#[adapt_model(storable)]
pub struct ParameterJoint {
    pub joint_type: Option<ParameterJointType>,
    pub filters: Option<Vec<ParameterCondition>>,
}
