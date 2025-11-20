use crate::{ComputedParameter, ConstantParameter, TopicFactorParameter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Parameter {
    #[serde(rename = "topic")]
    Topic(TopicFactorParameter),
    #[serde(rename = "constant")]
    Constant(ConstantParameter),
    #[serde(rename = "computed")]
    Computed(ComputedParameter),
}
