use crate::{ComputedParameter, ConstantParameter, TopicFactorParameter};
use elf_model_marco::VariousStructTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "kind")]
pub enum Parameter {
    #[serde(rename = "topic")]
    Topic(TopicFactorParameter),
    #[serde(rename = "constant")]
    Constant(ConstantParameter),
    #[serde(rename = "computed")]
    Computed(ComputedParameter),
}
