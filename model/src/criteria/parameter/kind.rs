use crate::serde_for_enum;
use std::fmt;

pub enum ParameterKind {
    Topic,
    Constant,
    Computed,
}

impl fmt::Display for ParameterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterKind::Topic => write!(f, "topic"),
            ParameterKind::Constant => write!(f, "constant"),
            ParameterKind::Computed => write!(f, "computed"),
        }
    }
}

serde_for_enum! {
    ParameterKind {
        Topic => "topic",
        Constant => "constant",
        Computed => "computed",
    }
}
