use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{AddParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcAddParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcAddParameter {}

impl ArcAddParameter {
    pub fn new(parameter: AddParameter) -> StdR<Arc<Self>> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[add] must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::Add),
            parameters: arc_parameters,
        }))
    }
}
