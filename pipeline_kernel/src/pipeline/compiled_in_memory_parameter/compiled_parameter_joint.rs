use crate::{CompiledParameterCondition, InMemoryData};
use elf_base::StdR;
use elf_model::{ParameterJointType, TenantId, TopicId};
use elf_runtime_model_kernel::{ArcParameterJoint, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

/// in-memory check
pub struct CompiledParameterJoint {
    r#type: Arc<ParameterJointType>,
    conditions: Vec<CompiledParameterCondition>,
}

impl CompiledParameterJoint {
    pub fn compile(
        value: &Arc<ArcParameterJoint>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let mut conditions = vec![];
        for filter in value.filters.deref() {
            conditions.push(CompiledParameterCondition::compile(
                filter,
                topic_schemas,
                tenant_id,
            )?)
        }

        Ok(CompiledParameterJoint {
            r#type: value.joint_type.clone(),
            conditions,
        })
    }
}

impl CompiledParameterJoint {
    pub fn is_true(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self.r#type.deref() {
            ParameterJointType::And => {
                // all are true == not any is false
                for condition in &self.conditions {
                    if condition.is_false(in_memory_data)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ParameterJointType::Or => {
                // any is true
                for condition in &self.conditions {
                    if condition.is_true(in_memory_data)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }

    /// override considering the performance when there are many conditions
    pub fn is_false(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        match self.r#type.deref() {
            ParameterJointType::And => {
                // any is false
                for condition in &self.conditions {
                    if condition.is_false(in_memory_data)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            ParameterJointType::Or => {
                // all are false == not any is true
                for condition in &self.conditions {
                    if condition.is_true(in_memory_data)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }
    }
}
