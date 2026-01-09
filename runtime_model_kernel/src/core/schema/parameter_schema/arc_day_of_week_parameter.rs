use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{DayOfWeekParameter, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcDayOfWeekParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcDayOfWeekParameter {}

impl ArcDayOfWeekParameter {
    pub fn new(parameter: DayOfWeekParameter) -> StdR<Arc<Self>> {
        let parameter =
            Self::must_then(parameter.parameter.map(|p| *p), ArcParameter::new, || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[day-of-week] must have sub parameter.")
            })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::DayOfWeek),
            parameter,
        }))
    }
}
