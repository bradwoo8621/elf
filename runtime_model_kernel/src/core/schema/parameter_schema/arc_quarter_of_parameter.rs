use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{ParameterComputeType, ParameterKind, QuarterOfParameter, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcQuarterOfParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcQuarterOfParameter {}

impl ArcQuarterOfParameter {
    pub fn new(parameter: QuarterOfParameter) -> StdR<Self> {
        let parameter = Self::must_then(
            parameter.parameter.map(|p| *p),
            ArcParameter::new_arc,
            || {
                RuntimeModelKernelErrorCode::ComputedParametersMissed
                    .msg("Computed parameter[quarter-of] must have sub parameter.")
            },
        )?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::QuarterOf),
            parameter,
        })
    }
}
