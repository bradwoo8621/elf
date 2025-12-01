use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    DayOfMonthParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR,
};

#[derive(Debug)]
pub struct ArcDayOfMonthParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcDayOfMonthParameter {}

impl ArcDayOfMonthParameter {
    pub fn new(parameter: DayOfMonthParameter) -> StdR<Self> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new_arc,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[day-of-month] must have sub parameter.")
            },
        )?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::DayOfMonth),
            parameter,
        })
    }
}
