use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::TopicDataValue;
use watchmen_runtime_model_kernel::ArcParameter;

pub enum CompiledParameter {
    Topic(CompiledTopicFactorParameter),
    Constant(CompiledConstantParameter),
    Computed(CompiledComputedParameter),
}

impl CompiledParameter {
    pub fn new(value: Arc<ArcParameter>) -> Self {
        match value.deref() {
            ArcParameter::Topic(v) => {
                CompiledParameter::Topic(CompiledTopicFactorParameter::new(v.clone()))
            }
            ArcParameter::Constant(v) => {
                CompiledParameter::Constant(CompiledConstantParameter::new(v.clone()))
            }
            ArcParameter::Computed(v) => {
                CompiledParameter::Computed(CompiledComputedParameter::new(v.clone()))
            }
        }
    }
}

impl InMemoryParameter for CompiledParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> &TopicDataValue {
        match self {
            CompiledParameter::Topic(v) => v.value_from(variables),
            CompiledParameter::Constant(v) => v.value_from(variables),
            CompiledParameter::Computed(v) => v.value_from(variables),
        }
    }
}
