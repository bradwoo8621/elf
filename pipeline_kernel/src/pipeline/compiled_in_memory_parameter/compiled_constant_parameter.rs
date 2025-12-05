use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::{StdR, TopicDataValue};
use watchmen_runtime_model_kernel::ArcConstantParameter;

pub struct CompiledConstantParameter {}

impl CompiledConstantParameter {
    pub fn new(_parameter: Arc<ArcConstantParameter>) -> StdR<Self> {
        Ok(CompiledConstantParameter {})
    }
}

impl InMemoryParameter for CompiledConstantParameter {
    fn value_from<'a>(
        &self,
        _variables: &'a PipelineExecutionVariables,
    ) -> StdR<&'a TopicDataValue> {
        todo!("implement value_from for CompiledConstantParameter")
    }
}
