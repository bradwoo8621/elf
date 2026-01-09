use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{MonthOfParameter, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcMonthOfParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcMonthOfParameter {}

impl ArcMonthOfParameter {
    pub fn new(parameter: MonthOfParameter) -> StdR<Arc<Self>> {
        let parameter =
            Self::must_then(parameter.parameter.map(|p| *p), ArcParameter::new, || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[month-of] must have sub parameter.")
            })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::MonthOf),
            parameter,
        }))
    }
}
