use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{DayOfWeekParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcDayOfWeekParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcDayOfWeekParameter {}

impl ArcDayOfWeekParameter {
    pub fn new(parameter: DayOfWeekParameter) -> StdR<Self> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new_arc,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[day-of-week] must have sub parameter.")
            },
        )?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::DayOfWeek),
            parameter,
        })
    }
}
