use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{ModulusParameter, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcModulusParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcModulusParameter {}

impl ArcModulusParameter {
    pub fn new(parameter: ModulusParameter) -> StdR<Arc<Self>> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[modulus] must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::Modulus),
            parameters: arc_parameters,
        }))
    }
}
