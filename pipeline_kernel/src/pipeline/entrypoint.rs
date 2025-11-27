use crate::{PipelineExecutionLogMonitor, PipelineKernelErrorCode, PipelineTrigger};
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerData, PipelineTriggerTraceId, PipelineTriggerType, StdErrorCode, StdR,
    StringUtils, TopicCode, TopicData, TopicDataId, UserRole, VoidR, VoidResultHelper,
};
use watchmen_runtime_model_kernel::{IdGenerator, TopicSchema};

struct CheckedPipelineTriggerData {
    /// topic name
    topic_schema: TopicSchema,
    /// current data
    trigger_data: TopicData,
    trigger_type: PipelineTriggerType,
    principal: Principal,
    trace_id: PipelineTriggerTraceId,
}

pub struct PipelineEntrypoint {
    principal: Principal,
    trace_id: Option<PipelineTriggerTraceId>,
}

impl PipelineEntrypoint {
    pub fn with(principal: Principal) -> Self {
        PipelineEntrypoint {
            principal,
            trace_id: None,
        }
    }

    pub fn traced_with(mut self, trace_id: PipelineTriggerTraceId) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    fn check_trigger_code(&self, trigger_data: &PipelineTriggerData) -> VoidR {
        if let Some(code) = &trigger_data.code {
            if code.is_blank() {
                PipelineKernelErrorCode::TriggerCodeIsBlank
                    .msg("Pipeline trigger code cannot be blank.")
            } else {
                Ok(())
            }
        } else {
            PipelineKernelErrorCode::TriggerCodeMissed.msg("Pipeline trigger code cannot be empty.")
        }
    }

    fn check_trigger_type(&self, trigger_data: &PipelineTriggerData) -> VoidR {
        if trigger_data.trigger_type.is_none() {
            PipelineKernelErrorCode::TriggerTypeMissed.msg("Pipeline trigger type cannot be empty.")
        } else {
            Ok(())
        }
    }

    fn check_trigger_data(&self, trigger_data: &PipelineTriggerData) -> VoidR {
        if trigger_data.data.is_none() {
            PipelineKernelErrorCode::TriggerDataMissed.msg("Pipeline trigger data cannot be empty.")
        } else {
            Ok(())
        }
    }

    fn check_trigger_access(&self, trigger_data: &PipelineTriggerData) -> VoidR {
        let principal = &self.principal;
        let opt_tenant_id = &trigger_data.tenant_id;

        if principal.is_super_admin() {
            if let Some(tenant_id) = opt_tenant_id {
                if tenant_id.is_blank() {
                    return PipelineKernelErrorCode::TriggerTenantIdIsBlank.msg(
                        "Pipeline trigger tenant id cannot be blank when triggered by super admin.",
                    );
                }
            } else {
                return PipelineKernelErrorCode::TriggerTenantIdMissed.msg(
                    "Pipeline trigger tenant id cannot be empty when triggered by super admin.",
                );
            }
        } else {
            if let Some(tenant_id) = opt_tenant_id {
                if tenant_id.is_not_blank() && *tenant_id != principal.tenant_id {
                    return PipelineKernelErrorCode::TriggerTenantIdMismatchPrincipal
                        .msg("Pipeline trigger tenant id does not match the principal's.");
                }
            }
        }
        Ok(())
    }

    fn find_topic_schema(&self, _code: TopicCode) -> StdR<TopicSchema> {
        todo!("implement find_topic_schema for PipelineEntrypoint")
    }

    fn ask_trace_id_generator(&self) -> StdR<Arc<dyn IdGenerator>> {
        todo!("implement ask_trace_id_generator for PipelineEntrypoint")
    }

    fn check_and_prepare(
        &self,
        trigger_data: PipelineTriggerData,
    ) -> StdR<CheckedPipelineTriggerData> {
        // check given data
        Vec::new()
            .collect(self.check_trigger_access(&trigger_data))
            .collect(self.check_trigger_code(&trigger_data))
            .collect(self.check_trigger_type(&trigger_data))
            .collect(self.check_trigger_data(&trigger_data))
            .accumulate()?;

        let topic_schema = self.find_topic_schema(trigger_data.code.unwrap())?;

        // prepare execute principal
        let execute_principal: Principal = if self.principal.is_super_admin() {
            // switch to given tenant and fake as admin role
            self.principal
                .switch_tenant(trigger_data.tenant_id.unwrap(), UserRole::Admin)
        } else {
            // use current principal
            self.principal.clone()
        };

        // prepare trace id
        let trace_id = if let Some(trace_id) = &self.trace_id {
            trace_id.clone()
        } else {
            self.ask_trace_id_generator()?.next_id().to_string()
        };

        Ok(CheckedPipelineTriggerData {
            topic_schema,
            trigger_data: trigger_data.data.unwrap(),
            trigger_type: trigger_data.trigger_type.unwrap(),
            principal: execute_principal,
            trace_id,
        })
    }

    fn create_trigger(&self, trigger_data: CheckedPipelineTriggerData) -> PipelineTrigger {
        let principal = Arc::new(trigger_data.principal);
        let trace_id = Arc::new(trigger_data.trace_id);

        PipelineTrigger {
            topic_schema: trigger_data.topic_schema,
            data: trigger_data.trigger_data,
            r#type: trigger_data.trigger_type,
            trace_id: trace_id.clone(),
            principal: principal.clone(),
            execution_log_monitor: PipelineExecutionLogMonitor {
                trace_id,
                principal,
            },
        }
    }

    pub fn execute(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        let checked_trigger_data = self.check_and_prepare(trigger_data)?;

        self.create_trigger(checked_trigger_data).execute()
    }

    pub async fn execute_async(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        let checked_trigger_data = self.check_and_prepare(trigger_data)?;

        self.create_trigger(checked_trigger_data)
            .execute_async()
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::PipelineEntrypoint;
    use watchmen_auth::Principal;
    use watchmen_model::{PipelineTriggerData, PipelineTriggerType};

    #[test]
    fn test() {
        let trigger_data = PipelineTriggerData::new()
            .code(String::from("topic-1"))
            .trigger_type(PipelineTriggerType::Insert)
            .tenant_id(String::from("tenant-1"));
        let result = PipelineEntrypoint::with(Principal::fake_super_admin())
            .traced_with("".to_string())
            .execute(trigger_data);
        assert!(result.is_ok());
    }
}
