use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{ParameterComputeType, ParameterKind, WeekOfYearParameter};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcWeekOfYearParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcWeekOfYearParameter {}

impl ArcWeekOfYearParameter {
    pub fn new(parameter: WeekOfYearParameter) -> StdR<Arc<Self>> {
        let parameter =
            Self::must_then(parameter.parameter.map(|p| *p), ArcParameter::new, || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[week-of-year] must have sub parameter.")
            })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::WeekOfYear),
            parameter,
        }))
    }
}
