use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    ParameterComputeType, ParameterKind, StdErrorCode, StdR, WeekOfMonthParameter,
};

#[derive(Debug)]
pub struct ArcWeekOfMonthParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcWeekOfMonthParameter {}

impl ArcWeekOfMonthParameter {
    pub fn new(parameter: WeekOfMonthParameter) -> StdR<Arc<Self>> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[week-of-month] must have sub parameter.")
            },
        )?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::WeekOfMonth),
            parameter,
        }))
    }
}
