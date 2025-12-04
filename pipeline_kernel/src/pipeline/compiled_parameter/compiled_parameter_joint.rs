use crate::{CompiledParameterCondition, InMemoryParameterCondition, PipelineExecutionVariables};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{ParameterJointType, StdR};
use watchmen_runtime_model_kernel::ArcParameterJoint;

/// in-memory check
pub struct CompiledParameterJoint {
    r#type: Arc<ParameterJointType>,
    conditions: Vec<CompiledParameterCondition>,
}

impl CompiledParameterJoint {
    pub fn new(value: Arc<ArcParameterJoint>) -> Self {
        CompiledParameterJoint {
            r#type: value.joint_type.clone(),
            conditions: value
                .filters
                .deref()
                .into_iter()
                .map(|f| CompiledParameterCondition::new(f.clone()))
                .collect(),
        }
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
