use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{MonthOfParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcMonthOfParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcMonthOfParameter {}

impl ArcMonthOfParameter {
    pub fn new(parameter: MonthOfParameter) -> StdR<Self> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new_arc,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[month-of] must have sub parameter.")
            },
        )?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::MonthOf),
            parameter,
        })
    }
}
