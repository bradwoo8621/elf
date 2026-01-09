use crate::{ArcHelper, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{ConstantParameter, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcConstantParameter {
    pub kind: Arc<ParameterKind>,
    pub value: Arc<String>,
}

impl ArcHelper for ArcConstantParameter {}

impl ArcConstantParameter {
    pub fn new(parameter: ConstantParameter) -> StdR<Arc<Self>> {
        let value = Self::not_blank(
            parameter.value,
            || {
                RuntimeModelKernelErrorCode::ParameterConstantValueMissed
                    .msg("Constant parameter must have a value.")
            },
            || {
                RuntimeModelKernelErrorCode::ParameterConstantValueIsBlank
                    .msg("Constant parameter's value cannot be blank.")
            },
        )?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Constant),
            value,
        }))
    }
}
