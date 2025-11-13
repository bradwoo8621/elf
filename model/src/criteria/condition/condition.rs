use crate::{BaseDataModel, ParameterExpression, ParameterJoint, Storable};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub enum ParameterCondition {
    Exp(ParameterExpression),
    Joint(ParameterJoint),
}
