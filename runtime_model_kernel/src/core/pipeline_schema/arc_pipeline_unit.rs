use crate::{ArcParameterJoint, ArcPipelineAction, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{PipelineUnit, PipelineUnitId, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcPipelineUnit {
    pub unit_id: Option<Arc<PipelineUnitId>>,
    pub name: Arc<String>,
    pub loop_variable_name: Option<Arc<String>>,
    pub r#do: Arc<Vec<Arc<ArcPipelineAction>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcPipelineUnit {
    pub fn new(unit: PipelineUnit) -> StdR<Arc<Self>> {
        // TIP a default name will be generated if there is no name on unit
        let name = Arc::new(unit.name.unwrap_or(String::from("unnamed-unit")));

        if unit.r#do.is_none() {
            return RuntimeModelKernelErrorCode::PipelineActionMissed
                .msg(format!("Pipeline unit[{}] has no action.", name));
        }
        let actions = unit.r#do.unwrap();
        if actions.len() == 0 {
            return RuntimeModelKernelErrorCode::PipelineActionMissed
                .msg(format!("Pipeline unit[{}] has no action.", name));
        }
        let mut arc_actions = vec![];
        for action in actions {
            arc_actions.push(ArcPipelineAction::new(action)?);
        }
        let arc_actions = Arc::new(arc_actions);

        let conditional = unit.conditional.unwrap_or(false);
        let on = if conditional {
            if unit.on.is_none() {
                return RuntimeModelKernelErrorCode::PipelineConditionMissed.msg(format!(
                    "Pipeline unit[{}] has no condition when conditional is true.",
                    name
                ));
            } else {
                Some(ArcParameterJoint::new(unit.on.unwrap())?)
            }
        } else {
            None
        };

        Ok(Arc::new(Self {
            unit_id: unit.unit_id.map(Arc::new),
            name,
            loop_variable_name: unit.loop_variable_name.map(Arc::new),
            r#do: arc_actions,
            conditional,
            on,
        }))
    }
}
