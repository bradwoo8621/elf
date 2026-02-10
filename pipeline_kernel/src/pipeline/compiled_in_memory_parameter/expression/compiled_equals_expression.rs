use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcEqualsExpression, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledEqualsExpression {
    pub fn compile(
        exp: &Arc<ArcEqualsExpression>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledEqualsExpression {
            left: CompiledParameter::compile(&exp.left, topic_schemas, tenant_id)?,
            right: CompiledParameter::compile(&exp.right, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledEqualsExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        Ok(self
            .left
            .value_from(in_memory_data)?
            .is_same_as(&self.right.value_from(in_memory_data)?.deref()))
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        Ok(self
            .left
            .value_from(in_memory_data)?
            .is_not_same_as(&self.right.value_from(in_memory_data)?.deref()))
    }
}
