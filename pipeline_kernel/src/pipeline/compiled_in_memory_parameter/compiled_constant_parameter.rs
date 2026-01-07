use crate::{ArcTopicDataValue, InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_base::StdR;
use watchmen_model::TenantId;
use watchmen_runtime_model_kernel::ArcConstantParameter;

pub struct CompiledConstantParameter;

impl CompiledConstantParameter {
    pub fn new(_parameter: &Arc<ArcConstantParameter>, _tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledConstantParameter {})
    }
}

impl InMemoryParameter for CompiledConstantParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement value_from for CompiledConstantParameter")
    }
}
