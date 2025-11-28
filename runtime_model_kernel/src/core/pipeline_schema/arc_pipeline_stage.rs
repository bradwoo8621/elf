use crate::{ArcParameterJoint, ArcPipelineUnit, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{PipelineStage, PipelineStageId, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcPipelineStage {
    pub stage_id: Option<Arc<PipelineStageId>>,
    pub name: Arc<String>,
    pub units: Arc<Vec<Arc<ArcPipelineUnit>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcPipelineStage {
    pub fn new(stage: PipelineStage) -> StdR<Arc<Self>> {
        // TIP a default name will be generated if there is no name on stage
        let name = Arc::new(stage.name.unwrap_or(String::from("unnamed-stage")));

        if stage.units.is_none() {
            return RuntimeModelKernelErrorCode::PipelineUnitMissed
                .msg(format!("Pipeline stage[{}] has no stage.", name));
        }
        let units = stage.units.unwrap();
        if units.len() == 0 {
            return RuntimeModelKernelErrorCode::PipelineUnitMissed
                .msg(format!("Pipeline stage[{}] has no stage.", name));
        }
        let mut arc_units = vec![];
        for unit in units {
            arc_units.push(ArcPipelineUnit::new(unit)?);
        }
        let arc_units = Arc::new(arc_units);

        let conditional = stage.conditional.unwrap_or(false);
        let on = if conditional {
            if stage.on.is_none() {
                return RuntimeModelKernelErrorCode::PipelineConditionMissed.msg(format!(
                    "Pipeline stage[{}] has no condition when conditional is true.",
                    name
                ));
            } else {
                Some(ArcParameterJoint::new(stage.on.unwrap())?)
            }
        } else {
            None
        };

        Ok(Arc::new(Self {
            stage_id: stage.stage_id.map(Arc::new),
            name,
            units: arc_units,
            conditional,
            on,
        }))
    }
}
