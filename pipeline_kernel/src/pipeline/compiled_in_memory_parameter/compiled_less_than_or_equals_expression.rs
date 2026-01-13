use crate::{CompiledParameter, InMemoryData};
use elf_base::StdR;
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcLessThanOrEqualsExpression;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledLessThanOrEqualsExpression {
    left: CompiledParameter,
    right: CompiledParameter,
}

impl CompiledLessThanOrEqualsExpression {
    pub fn compile(exp: &Arc<ArcLessThanOrEqualsExpression>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledLessThanOrEqualsExpression {
            left: CompiledParameter::compile(&exp.left, tenant_id)?,
            right: CompiledParameter::compile(&exp.right, tenant_id)?,
        })
    }
}

impl CompiledLessThanOrEqualsExpression {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_less_than_or_equals(self.right.value_from(in_memory_data)?.deref())
    }

    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.left
            .value_from(in_memory_data)?
            .is_more_than(self.right.value_from(in_memory_data)?.deref())
    }
}
