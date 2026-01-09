use crate::{CompiledParameterCondition, InMemoryParameterCondition, PipelineExecutionVariables};
use elf_base::StdR;
use elf_model::{ParameterJointType, TenantId};
use elf_runtime_model_kernel::ArcParameterJoint;
use std::ops::Deref;
use std::sync::Arc;

/// in-memory check
pub struct CompiledParameterJoint {
    r#type: Arc<ParameterJointType>,
    conditions: Vec<CompiledParameterCondition>,
}

impl CompiledParameterJoint {
    pub fn new(value: &Arc<ArcParameterJoint>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        let mut conditions = vec![];
        for filter in value.filters.deref() {
            conditions.push(CompiledParameterCondition::new(filter, tenant_id)?)
        }

        Ok(CompiledParameterJoint {
            r#type: value.joint_type.clone(),
            conditions,
        })
    }
}

impl InMemoryParameterCondition for CompiledParameterJoint {
    fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self.r#type.deref() {
            ParameterJointType::And => {
                // all are true == not any is false
                for condition in &self.conditions {
                    if condition.is_false(variables)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ParameterJointType::Or => {
                // any is true
                for condition in &self.conditions {
                    if condition.is_true(variables)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }

    /// override considering the performance when there are many conditions
    fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        match self.r#type.deref() {
            ParameterJointType::And => {
                // any is false
                for condition in &self.conditions {
                    if condition.is_false(variables)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            ParameterJointType::Or => {
                // all are false == not any is true
                for condition in &self.conditions {
                    if condition.is_true(variables)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }
    }
}
