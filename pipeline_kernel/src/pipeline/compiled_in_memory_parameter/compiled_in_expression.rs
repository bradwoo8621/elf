use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcInExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledInExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledInExpression {
    pub fn compile(exp: &Arc<ArcInExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledInExpression {
            left: CompiledParameter::compile(&exp.left, tenant_id)?,
            right: CompiledParameter::compile(&exp.right, tenant_id)?,
        })
    }
}

impl CompiledInExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_in(self.right.value_from(in_memory_data)?.deref())
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_not_in(self.right.value_from(in_memory_data)?.deref())
    }
}
