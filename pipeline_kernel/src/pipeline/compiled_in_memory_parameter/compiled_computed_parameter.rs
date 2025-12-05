use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId, TopicDataValue};
use watchmen_runtime_model_kernel::ArcComputedParameter;

pub struct CompiledComputedParameter {}

impl CompiledComputedParameter {
    pub fn new(_parameter: &Arc<ArcComputedParameter>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledComputedParameter {})
    }
}

impl InMemoryParameter for CompiledComputedParameter {
    fn value_from<'a>(
        &self,
        _variables: &'a PipelineExecutionVariables,
    ) -> StdR<&'a TopicDataValue> {
        todo!("implement value_from for CompiledComputedParameter")
    }
}
