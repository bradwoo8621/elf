use crate::{ArcHelper, ArcParameterJoint, ArcPipelineAction, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{PipelineUnit, PipelineUnitId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcPipelineUnit {
    pub unit_id: Arc<PipelineUnitId>,
    pub name: Arc<String>,
    pub loop_variable_name: Option<Arc<String>>,
    pub r#do: Arc<Vec<Arc<ArcPipelineAction>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcHelper for ArcPipelineUnit {}

impl ArcPipelineUnit {
    pub fn new(unit: PipelineUnit) -> StdR<Arc<Self>> {
        let unit_id = Self::or_empty_str(unit.unit_id);
        // TIP a default name will be generated if there is no name on unit
        let name = Arc::new(unit.name.unwrap_or(String::from("unnamed-unit")));
        let arc_actions = Self::must_vec(unit.r#do, ArcPipelineAction::new, || {
            RuntimeModelKernelErrorCode::PipelineActionMissed
                .msg(format!("Pipeline unit[{}] must have action.", unit_id))
        })?;
        let on = Self::conditional(unit.conditional, unit.on, || {
            format!(
                "Pipeline unit[{}] must have condition when conditional is true.",
                unit_id
            )
        })?;

        Ok(Arc::new(Self {
            unit_id,
            name,
            loop_variable_name: unit.loop_variable_name.map(Arc::new),
            r#do: arc_actions,
            conditional: on.is_some(),
            on,
        }))
    }
}
