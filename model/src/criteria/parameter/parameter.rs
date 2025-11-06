use crate::{
    BaseDataModel, ComputedParameter, ConstantParameter, ParameterKind, TopicFactorParameter,
};
use watchmen_model_marco::adapt_model;

#[adapt_model(bdm)]
pub enum Parameter {
    Topic(TopicFactorParameter),
    Constant(ConstantParameter),
    Computed(ComputedParameter),
}

impl Parameter {
    pub fn kind(&self) -> ParameterKind {
        match self {
            Parameter::Topic(_) => ParameterKind::Topic,
            Parameter::Constant(_) => ParameterKind::Constant,
            Parameter::Computed(_) => ParameterKind::Computed,
        }
    }
}
