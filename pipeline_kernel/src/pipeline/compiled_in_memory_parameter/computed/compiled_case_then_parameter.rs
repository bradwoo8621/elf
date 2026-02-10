use crate::{ArcTopicDataValue, CompiledParameter, CompiledParameterJoint, InMemoryData};
use elf_base::{ErrorCode, StdR};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcCaseThenParameter, RuntimeModelKernelErrorCode, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledCaseThenParameter {
    routes: Vec<(CompiledParameterJoint, CompiledParameter)>,
    default_route: Option<CompiledParameter>,
}

impl CompiledCaseThenParameter {
    pub fn compile(
        param: &Arc<ArcCaseThenParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let mut routes = vec![];
        let mut default_route = None;
        for parameter in param.parameters.iter() {
            if parameter.conditional && parameter.on.is_some() {
                routes.push((
                    CompiledParameterJoint::compile(
                        &parameter.on.as_ref().unwrap(),
                        topic_schemas,
                        tenant_id,
                    )?,
                    CompiledParameter::compile(&parameter.parameter, topic_schemas, tenant_id)?,
                ))
            } else if default_route.is_some() {
                return RuntimeModelKernelErrorCode::ComputedParametersMissed.msg(
                    "Computed parameter[case-then] can be at most one route without condition.",
                );
            } else {
                default_route = Some(CompiledParameter::compile(
                    &parameter.parameter,
                    topic_schemas,
                    tenant_id,
                )?)
            }
        }

        Ok(CompiledCaseThenParameter {
            routes,
            default_route,
        })
    }
}

impl CompiledCaseThenParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        for (joint, value) in self.routes.iter() {
            if joint.is_true(in_memory_data)? {
                return value.value_from(in_memory_data);
            }
        }

        if let Some(default_route) = &self.default_route {
            default_route.value_from(in_memory_data)
        } else {
            Ok(Arc::new(ArcTopicDataValue::None))
        }
    }
}
