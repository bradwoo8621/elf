use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcLessThanExpression, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledLessThanExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanExpression {
    pub fn compile(
        exp: &Arc<ArcLessThanExpression>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledLessThanExpression {
            left: CompiledParameter::compile(&exp.left, topic_schemas, tenant_id)?,
            right: CompiledParameter::compile(&exp.right, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledLessThanExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_less_than(self.right.value_from(in_memory_data)?.deref())
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_more_than_or_equals(self.right.value_from(in_memory_data)?.deref())
    }
}
