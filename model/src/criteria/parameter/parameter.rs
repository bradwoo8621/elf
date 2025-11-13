use crate::{
    BaseDataModel, ComputedParameter, ConstantParameter, ParameterKind, Storable,
    TopicFactorParameter,
};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
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
