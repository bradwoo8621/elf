use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{NoneParameter, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcNoneParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcParameter>>>,
}

impl ArcHelper for ArcNoneParameter {}

impl ArcNoneParameter {
    pub fn new(parameter: NoneParameter) -> StdR<Arc<Self>> {
        let arc_parameters = Self::must_vec(parameter.parameters, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[none] must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::None),
            parameters: arc_parameters,
        }))
    }
}
