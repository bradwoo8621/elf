use super::ComputedParameter;
use super::ConstantParameter;
use super::ParameterKind;
use super::TopicFactorParameter;
use crate::BaseDataModel;

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

impl BaseDataModel for Parameter {}
