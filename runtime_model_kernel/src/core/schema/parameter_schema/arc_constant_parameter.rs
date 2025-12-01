use crate::{ArcHelper, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{ConstantParameter, ParameterKind, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcConstantParameter {
    pub kind: Arc<ParameterKind>,
    pub value: Arc<String>,
}

impl ArcHelper for ArcConstantParameter {}

impl ArcConstantParameter {
    pub fn new(parameter: ConstantParameter) -> StdR<Self> {
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

        Ok(Self {
            kind: Arc::new(ParameterKind::Constant),
            value,
        })
    }
}
