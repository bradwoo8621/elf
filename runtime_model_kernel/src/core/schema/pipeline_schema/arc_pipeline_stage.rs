use crate::{ArcHelper, ArcParameterJoint, ArcPipelineUnit, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{PipelineStage, PipelineStageId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcPipelineStage {
    pub stage_id: Arc<PipelineStageId>,
    pub name: Arc<String>,
    pub units: Arc<Vec<Arc<ArcPipelineUnit>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcHelper for ArcPipelineStage {}

impl ArcPipelineStage {
    pub fn new(stage: PipelineStage) -> StdR<Arc<Self>> {
        let stage_id = Self::or_empty_str(stage.stage_id);
        // TIP a default name will be generated if there is no name on stage
        let name = Arc::new(stage.name.unwrap_or(String::from("unnamed-stage")));
        let arc_units = Self::must_vec(stage.units, ArcPipelineUnit::new, || {
            RuntimeModelKernelErrorCode::PipelineUnitMissed
                .msg(format!("Pipeline stage[{}] must have unit.", stage_id))
        })?;
        let on = Self::conditional(stage.conditional, stage.on, || {
            format!(
                "Pipeline stage[{}] must have condition when conditional is true.",
                stage_id
            )
        })?;

        Ok(Arc::new(Self {
            stage_id,
            name,
            units: arc_units,
            conditional: on.is_some(),
            on,
        }))
    }
}
