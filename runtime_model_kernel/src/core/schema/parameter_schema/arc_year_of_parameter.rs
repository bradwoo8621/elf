use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{ParameterComputeType, ParameterKind, StdErrorCode, StdR, YearOfParameter};

#[derive(Debug)]
pub struct ArcYearOfParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcYearOfParameter {}

impl ArcYearOfParameter {
    pub fn new(parameter: YearOfParameter) -> StdR<Self> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new_arc,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[year-of] must have sub parameter.")
            },
        )?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::YearOf),
            parameter,
        })
    }
}
