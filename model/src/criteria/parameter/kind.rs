use crate::serde_for_enum;
use watchmen_model_marco::Display;

#[derive(Display)]
pub enum ParameterKind {
    Topic,
    Constant,
    Computed,
}

serde_for_enum! {
    ParameterKind {
        Topic => "topic",
        Constant => "constant",
        Computed => "computed",
    }
}
