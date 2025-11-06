use crate::{BaseDataModel, ParameterExpression, ParameterJoint};
use watchmen_model_marco::adapt_model;

#[adapt_model(bdm)]
pub enum ParameterCondition {
    Exp(ParameterExpression),
    Joint(ParameterJoint),
}
