use watchmen_model_marco::{Display, Serde};

#[derive(Display, Serde)]
pub enum ParameterKind {
    Topic,
    Constant,
    Computed,
}
