use crate::{CompiledParameterJoint, InMemoryParameterCondition, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::ArcParameterJoint;

/// in-memory check
pub struct CompiledConditional {
    /// is some only when should is true, otherwise is none
    inner: Option<CompiledParameterJoint>,
}

impl CompiledConditional {
    pub fn new(conditional: Option<Arc<ArcParameterJoint>>) -> Self {
        if let Some(conditional) = &conditional {
            CompiledConditional {
                inner: Some(CompiledParameterJoint::new(conditional.clone())),
            }
        } else {
            CompiledConditional { inner: None }
        }
    }

    pub fn is_true(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_true(variables))
            // returns true when no condition
            .unwrap_or(Ok(true))
    }

    pub fn is_false(&self, variables: &PipelineExecutionVariables) -> StdR<bool> {
        self.inner
            .as_ref()
            .map(|inner| inner.is_false(variables))
            // returns false when no condition
            .unwrap_or(Ok(false))
    }
}
