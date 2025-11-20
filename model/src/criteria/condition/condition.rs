use crate::{ParameterExpression, ParameterJoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterCondition {
    Expression(ParameterExpression),
    Joint(ParameterJoint),
}
