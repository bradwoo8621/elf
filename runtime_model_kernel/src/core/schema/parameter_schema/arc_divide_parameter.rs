use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{DivideParameter, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcDivideParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcDivideParameter {}

impl ArcDivideParameter {
    pub fn new(parameter: DivideParameter) -> StdR<Arc<Self>> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[divide] must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::Divide),
            parameters: arc_parameters,
        }))
    }
}
