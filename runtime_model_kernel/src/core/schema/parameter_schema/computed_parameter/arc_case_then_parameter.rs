use crate::{ArcHelper, ArcParameter, ArcParameterJoint, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{CaseThenParameter, CaseThenParameterRoute, ParameterComputeType, ParameterKind};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcCaseThenParameterRoute {
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
    pub parameter: Arc<ArcParameter>,
}

impl ArcHelper for ArcCaseThenParameterRoute {}

impl ArcCaseThenParameterRoute {
    pub fn new(route: CaseThenParameterRoute) -> StdR<Arc<Self>> {
        let on = Self::conditional(
            route.conditional,
            route.on,
            || "Case then route must have condition when conditional is true.",
        )?;
        let parameter = Self::must_then(route.parameter, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::CaseThenRouteParameterMissed
                .msg("Case then route must have sub parameter.")
        })?;

        Ok(Arc::new(Self {
            conditional: on.is_some(),
            on,
            parameter,
        }))
    }
}

#[derive(Debug)]
pub struct ArcCaseThenParameter {
    pub kind: Arc<ParameterKind>,
    pub r#type: Arc<ParameterComputeType>,
    pub parameters: Arc<Vec<Arc<ArcCaseThenParameterRoute>>>,
}

impl ArcHelper for ArcCaseThenParameter {}

impl ArcCaseThenParameter {
    pub fn new(parameter: CaseThenParameter) -> StdR<Arc<Self>> {
        let values = if let Some(parameters) = parameter.parameters {
            parameters
        } else {
            return RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[case-then] must have sub parameter.");
        };
        if values.len() == 0 {
            return RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[case-then] must have sub parameter.");
        }

        let mut default_route_count = 0;
        for route in &values {
            if route.conditional.unwrap_or(false) {
                default_route_count += 1;
            }
        }
        if default_route_count > 1 {
            return RuntimeModelKernelErrorCode::ComputedParametersMissed
                .msg("Computed parameter[case-then] can be at most one route without condition.");
        }

        // transform and put the default route to last
        let mut arc_default_route: Option<Arc<ArcCaseThenParameterRoute>> = None;
        let mut arc_parameters = vec![];
        for value in values {
            let arc_parameter = ArcCaseThenParameterRoute::new(value)?;
            if arc_parameter.conditional {
                arc_parameters.push(arc_parameter);
            } else {
                arc_default_route = Some(arc_parameter);
            }
        }
        if let Some(route) = arc_default_route {
            arc_parameters.push(route);
        }
        let arc_parameters = Arc::new(arc_parameters);

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Computed),
            r#type: Arc::new(ParameterComputeType::CaseThen),
            parameters: arc_parameters,
        }))
    }
}
