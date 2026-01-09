use crate::{ArcComputedParameter, ArcConstantParameter, ArcTopicFactorParameter};
use elf_base::StdR;
use elf_model::Parameter;
use std::sync::Arc;

#[derive(Debug)]
pub enum ArcParameter {
    Topic(Arc<ArcTopicFactorParameter>),
    Constant(Arc<ArcConstantParameter>),
    Computed(Arc<ArcComputedParameter>),
}

impl ArcParameter {
    pub fn new(parameter: Parameter) -> StdR<Arc<Self>> {
        let arc_parameter = match parameter {
            Parameter::Topic(p) => ArcParameter::Topic(ArcTopicFactorParameter::new(p)?),
            Parameter::Constant(p) => ArcParameter::Constant(ArcConstantParameter::new(p)?),
            Parameter::Computed(p) => ArcParameter::Computed(ArcComputedParameter::new(p)?),
        };

        Ok(Arc::new(arc_parameter))
    }
}
