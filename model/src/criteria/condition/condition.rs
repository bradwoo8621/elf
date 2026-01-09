use crate::{ParameterExpression, ParameterJoint};
use elf_model_marco::VariousStructTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ParameterCondition {
    Expression(ParameterExpression),
    Joint(ParameterJoint),
}
