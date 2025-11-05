use super::ParameterExpression;
use super::ParameterJoint;
use crate::BaseDataModel;

pub enum ParameterCondition {
    Exp(ParameterExpression),
    Joint(ParameterJoint),
}

impl BaseDataModel for ParameterCondition {}
