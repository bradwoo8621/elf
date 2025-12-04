use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    HalfYearOfParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR,
};

#[derive(Debug)]
pub struct ArcHalfYearOfParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcHalfYearOfParameter {}

impl ArcHalfYearOfParameter {
    pub fn new(parameter: HalfYearOfParameter) -> StdR<Arc<Self>> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[half-year-of] must have sub parameter.")
            },
        )?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::HalfYearOf),
            parameter,
        }))
    }
}
