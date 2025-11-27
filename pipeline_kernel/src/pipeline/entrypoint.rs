use crate::PipelineKernelErrorCode;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerData, PipelineTriggerTraceId, StdErrorCode, StdR, StringUtils, VoidResultHelper,
    TopicDataId, VoidR,
};

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
            if opt_tenant_id.is_none() {
                return PipelineKernelErrorCode::TriggerTenantIdMissed.msg(
                    "Pipeline trigger tenant id cannot be empty when triggered by super admin.",
                );
            }
        } else {
        }
        Ok(())
    }

    fn check(&self, trigger_data: &PipelineTriggerData) -> VoidR {
        Vec::new()
            .collect(self.check_trigger_access(trigger_data))
            .collect(self.check_trigger_code(trigger_data))
            .collect(self.check_trigger_type(trigger_data))
            .collect(self.check_trigger_data(trigger_data))
            .accumulate()
    }

    pub fn execute(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        self.check(&trigger_data)?;

        todo!("implement execute for PipelineEntrypoint")
    }

    pub async fn execute_async(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        self.check(&trigger_data)?;

        todo!("implement execute_async for PipelineEntrypoint")
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
