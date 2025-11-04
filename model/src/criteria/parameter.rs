use super::{parameter_joint::ParameterJoint, parameter_kind::ParameterKind};

pub trait Parameter {
    fn kind(&self) -> ParameterKind;
}

pub trait ConditionalParameter: Parameter {
    fn conditional(&self) -> Option<bool>;
    fn on(&self) -> Option<Box<dyn ParameterJoint>>;
}
