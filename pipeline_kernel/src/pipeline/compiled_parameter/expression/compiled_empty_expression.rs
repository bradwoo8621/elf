use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcEmptyExpression, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledEmptyExpression {
    left: CompiledParameter,
}

impl CompiledEmptyExpression {
    pub fn compile(
        exp: &Arc<ArcEmptyExpression>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledEmptyExpression {
            left: CompiledParameter::compile(&exp.left, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledEmptyExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        Ok(self.left.value_from(in_memory_data)?.is_empty())
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        Ok(self.left.value_from(in_memory_data)?.is_not_empty())
    }
}
