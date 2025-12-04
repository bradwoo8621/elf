use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{ParameterComputeType, ParameterKind, StdErrorCode, StdR, SubtractParameter};

#[derive(Debug)]
pub struct ArcSubtractParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcSubtractParameter {}

impl ArcSubtractParameter {
    pub fn new(parameter: SubtractParameter) -> StdR<Arc<Self>> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[subtract] must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::Subtract),
            parameters: arc_parameters,
        }))
    }
}
