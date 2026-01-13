use crate::{PipelineExecutionLogMonitor, PipelineKernelErrorCode, PipelineTrigger};
use elf_auth::Principal;
use elf_base::VoidResultHelper;
use elf_base::{ErrorCode, StdR, StringUtils, VoidR};
use elf_model::{
    PipelineId, PipelineTriggerData, PipelineTriggerTraceId, PipelineTriggerType, TopicData,
    TopicDataId, UserRole,
};
use elf_runtime_model_kernel::{IdGen, TopicSchema, TopicSchemaProvider, TopicService};
use std::sync::Arc;

/// This is the main entry point for executing pipelines.
/// At this point, the specific pipelines to be executed are not yet known.
/// Need to provide:
/// - The topic name (code),
/// - The corresponding topic data for above topic,
/// - The type of trigger to be executed,
/// - The principal information of the executor,
/// - The trace id is used as the basis for linking all processes in a single pipelines execution.
///   If this execution is caused by the execution of another pipeline,
///   then the trace id needs to be provided; otherwise, it is not required.
pub struct PipelineEntrypoint {
    principal: Principal,
    pipeline_id: Option<PipelineId>,
    trace_id: Option<PipelineTriggerTraceId>,
}

impl PipelineEntrypoint {
    pub fn with(principal: Principal) -> Self {
        Self {
            principal,
            pipeline_id: None,
            trace_id: None,
        }
    }

    pub fn pipeline(mut self, pipeline_id: PipelineId) -> StdR<Self> {
        if pipeline_id.is_blank() {
            PipelineKernelErrorCode::TriggerPipelineIdIsBlank
                .msg("Given pipeline id cannot be blank.")
        } else {
            self.pipeline_id = Some(pipeline_id);
            Ok(self)
        }
    }

    pub fn traced_with(mut self, trace_id: PipelineTriggerTraceId) -> StdR<Self> {
        if trace_id.is_blank() {
            PipelineKernelErrorCode::TriggerTraceIdIsBlank.msg("Given pipeline id cannot be blank.")
        } else {
            self.trace_id = Some(trace_id);
            Ok(self)
        }
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

    fn check_trigger_type_with_topic(
        &self,
        trigger_data: &PipelineTriggerData,
        topic_schema: &Arc<TopicSchema>,
    ) -> VoidR {
        match &trigger_data.trigger_type {
            Some(trigger_type) => match trigger_type {
                PipelineTriggerType::Insert => Ok(()),
                other => {
                    let topic = topic_schema.topic();
                    if topic.is_synonym_topic() {
                        PipelineKernelErrorCode::TriggerTypeNotSupportedOnSynonym.msg(format!(
                            "Trigger type[{}] is not supported on synonym[{}].",
                            other,
                            topic_schema.name()
                        ))
                    } else if topic.is_raw_topic() {
                        PipelineKernelErrorCode::TriggerTypeNotSupportedOnRaw.msg(format!(
                            "Trigger type[{}] is not supported on raw[{}].",
                            other,
                            topic_schema.name()
                        ))
                    } else {
                        Ok(())
                    }
                }
            },
            _ => PipelineKernelErrorCode::TriggerTypeMissed
                .msg("Pipeline trigger type cannot be empty."),
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

    fn check_and_prepare(
        &self,
        trigger_data: PipelineTriggerData,
    ) -> StdR<(PipelineTrigger, TopicData)> {
        // check given data
        Vec::new()
            .collect(self.check_trigger_access(&trigger_data))
            .collect(self.check_trigger_code(&trigger_data))
            .collect(self.check_trigger_type(&trigger_data))
            .collect(self.check_trigger_data(&trigger_data))
            .accumulate()?;

        // prepare execute principal
        let execute_principal: Principal = if self.principal.is_super_admin() {
            // switch to given tenant and fake as admin role
            let trigger_tenant_id = trigger_data.tenant_id.clone().unwrap();
            self.principal
                .switch_tenant(trigger_tenant_id, UserRole::Admin)
        } else {
            // use current principal
            self.principal.clone()
        };

        let topic_schema = TopicService::schema()?.by_code(
            &trigger_data.code.as_ref().unwrap(),
            &execute_principal.tenant_id,
        )?;
        self.check_trigger_type_with_topic(&trigger_data, &topic_schema)?;

        // prepare trace id
        let trace_id = if let Some(trace_id) = &self.trace_id {
            trace_id.clone()
        } else {
            IdGen::next_id()?.to_string()
        };

        let principal = Arc::new(execute_principal);
        let trace_id = Arc::new(trace_id);
        let pipeline_trigger = PipelineTrigger {
            pipeline_id: self.pipeline_id.clone(),
            topic_schema,
            r#type: trigger_data.trigger_type.unwrap(),
            trace_id: trace_id.clone(),
            principal: principal.clone(),
            execution_log_monitor: Arc::new(PipelineExecutionLogMonitor {
                trace_id,
                principal,
            }),
        };
        let topic_data = trigger_data.data.unwrap();
        Ok((pipeline_trigger, topic_data))
    }

    pub fn execute(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        let (pipeline_trigger, topic_data) = self.check_and_prepare(trigger_data)?;
        pipeline_trigger.execute(topic_data)
    }

    pub async fn execute_async(&self, trigger_data: PipelineTriggerData) -> StdR<TopicDataId> {
        let (pipeline_trigger, topic_data) = self.check_and_prepare(trigger_data)?;
        pipeline_trigger.execute_async(topic_data).await
    }
}

#[cfg(test)]
mod tests {
    use crate::PipelineEntrypoint;
    use elf_auth::Principal;
    use elf_model::{PipelineTriggerData, PipelineTriggerType, TopicData};

    #[test]
    fn test() {
        let topic_data = TopicData::new();
        let trigger_data = PipelineTriggerData::new()
            .code(String::from("topic-1"))
            .trigger_type(PipelineTriggerType::Insert)
            .tenant_id(String::from("tenant-1"))
            .data(topic_data);
        let result = PipelineEntrypoint::with(Principal::fake_super_admin())
            .traced_with("123".to_string())
            .expect("trace id cannot be blank")
            .execute(trigger_data);
        assert!(result.is_ok());
    }
}
