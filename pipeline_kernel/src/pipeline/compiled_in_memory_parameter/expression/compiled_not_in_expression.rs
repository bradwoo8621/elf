use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcNotInExpression, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledNotInExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledNotInExpression {
    pub fn compile(
        exp: &Arc<ArcNotInExpression>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledNotInExpression {
            left: CompiledParameter::compile(&exp.left, topic_schemas, tenant_id)?,
            right: CompiledParameter::compile(&exp.right, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledNotInExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_not_in(self.right.value_from(in_memory_data)?.deref())
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_in(self.right.value_from(in_memory_data)?.deref())
    }
}
