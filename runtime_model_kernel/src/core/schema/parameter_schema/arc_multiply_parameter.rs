use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{MultiplyParameter, ParameterComputeType, ParameterKind, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcMultiplyParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcMultiplyParameter {}

impl ArcMultiplyParameter {
    pub fn new(parameter: MultiplyParameter) -> StdR<Self> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new_arc, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[multiply] must have sub parameter.")
        })?;

        Ok(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::Multiply),
            parameters: arc_parameters,
        })
    }
}
