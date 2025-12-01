use crate::{ArcComputedParameter, ArcConstantParameter, ArcTopicFactorParameter};
use std::sync::Arc;
use watchmen_model::{Parameter, StdR};

#[derive(Debug)]
pub enum ArcParameter {
    Topic(ArcTopicFactorParameter),
    Constant(ArcConstantParameter),
    Computed(ArcComputedParameter),
}

impl ArcParameter {
    pub fn new_arc(parameter: Parameter) -> StdR<Arc<Self>> {
        let arc_parameter = match parameter {
            Parameter::Topic(p) => ArcParameter::Topic(ArcTopicFactorParameter::new(p)?),
            Parameter::Constant(p) => ArcParameter::Constant(ArcConstantParameter::new(p)?),
            Parameter::Computed(p) => ArcParameter::Computed(ArcComputedParameter::new(p)?),
        };

        Ok(Arc::new(arc_parameter))
    }
}
