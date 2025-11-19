use crate::{BaseDataModel, ParameterExpression, ParameterJoint, Storable};
use watchmen_model_marco::adapt_model;

/// TODO is it workable?
#[adapt_model(storable)]
pub enum ParameterCondition {
    Expression(ParameterExpression),
    Joint(ParameterJoint),
}
