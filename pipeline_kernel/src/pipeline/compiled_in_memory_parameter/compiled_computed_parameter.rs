use crate::{ArcTopicDataValue, InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_base::StdR;
use watchmen_model::TenantId;
use watchmen_runtime_model_kernel::ArcComputedParameter;

pub struct CompiledComputedParameter;

impl CompiledComputedParameter {
    pub fn new(_parameter: &Arc<ArcComputedParameter>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledComputedParameter {})
    }
}

impl InMemoryParameter for CompiledComputedParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledComputedParameter")
    }
}
