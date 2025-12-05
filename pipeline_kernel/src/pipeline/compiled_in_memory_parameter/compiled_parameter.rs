use crate::{
    CompiledComputedParameter, CompiledConstantParameter, CompiledTopicFactorParameter,
    InMemoryParameter, PipelineExecutionVariables,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{StdR, TopicDataValue};
use watchmen_runtime_model_kernel::ArcParameter;

pub enum CompiledParameter {
    Topic(CompiledTopicFactorParameter),
    Constant(CompiledConstantParameter),
    Computed(CompiledComputedParameter),
}

impl CompiledParameter {
    pub fn new(value: Arc<ArcParameter>) -> StdR<Self> {
        match value.deref() {
            ArcParameter::Topic(v) => {
                CompiledTopicFactorParameter::new(v.clone()).map(|p| CompiledParameter::Topic(p))
            }
            ArcParameter::Constant(v) => {
                CompiledConstantParameter::new(v.clone()).map(|p| CompiledParameter::Constant(p))
            }
            ArcParameter::Computed(v) => {
                CompiledComputedParameter::new(v.clone()).map(|p| CompiledParameter::Computed(p))
            }
        }
    }
}

impl InMemoryParameter for CompiledParameter {
    fn value_from<'a>(
        &self,
        variables: &'a PipelineExecutionVariables,
    ) -> StdR<&'a TopicDataValue> {
        match self {
            CompiledParameter::Topic(v) => v.value_from(variables),
            CompiledParameter::Constant(v) => v.value_from(variables),
            CompiledParameter::Computed(v) => v.value_from(variables),
        }
    }
}
