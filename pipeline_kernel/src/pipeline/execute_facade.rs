use crate::{PipelineExecuteInput, PipelineExecutor, PipelineKernelErrorCode, PipelinePreExecute};
use elf_auth::Principal;
use elf_base::{ErrorCode, StdR, StringUtils, VoidResultHelper};
use elf_model::{
    PipelineId, PipelineTriggerData, PipelineTriggerTraceId, PipelineTriggerType, TenantId,
    TopicCode, TopicData, TopicDataId, UserRole,
};
use elf_runtime_model_kernel::{IdGen, TopicSchema, TopicSchemaProvider, TopicService};
use std::sync::Arc;

pub struct PipelineExecuteFacade {
    input: PipelineExecuteInput,
}

impl PipelineExecuteFacade {
    /// - when principal is super admin, tenant id must be provided by trigger data, and return it,
    /// - when principal is not super admin, and tenant id not provided by trigger data,
    ///   returns principal's tenant id,
    /// - when principal is not super admin, and tenant id is provided by trigger data, and they are same,
    ///   returns principal's tenant id,
    fn check_tenant(principal: &Principal, trigger_data: &PipelineTriggerData) -> StdR<TenantId> {
        let execute_tenant_id = &trigger_data.tenant_id;

        if principal.is_super_admin() {
            if let Some(tenant_id) = execute_tenant_id {
                if tenant_id.is_blank() {
                    PipelineKernelErrorCode::TriggerTenantIdIsBlank.msg(
                        "Pipeline trigger tenant id cannot be blank when triggered by super admin.",
                    )
                } else {
                    Ok(tenant_id.clone())
                }
            } else {
                PipelineKernelErrorCode::TriggerTenantIdMissed.msg(
                    "Pipeline trigger tenant id cannot be empty when triggered by super admin.",
                )
            }
        } else {
            if let Some(tenant_id) = execute_tenant_id {
                if tenant_id.is_not_blank() && tenant_id != principal.tenant_id() {
                    PipelineKernelErrorCode::TriggerTenantIdMismatchPrincipal
                        .msg("Pipeline trigger tenant id does not match the principal's.")
                } else {
                    Ok(tenant_id.clone())
                }
            } else {
                Ok(principal.tenant_id().clone())
            }
        }
    }

    /// trigger code (topic code) must be provided, and can not be blank.
    fn check_trigger_code(trigger_data: &PipelineTriggerData) -> StdR<TopicCode> {
        if let Some(code) = &trigger_data.code {
            if code.is_blank() {
                PipelineKernelErrorCode::TriggerCodeIsBlank
                    .msg("Pipeline trigger code cannot be blank.")
            } else {
                Ok(code.clone())
            }
        } else {
            PipelineKernelErrorCode::TriggerCodeMissed.msg("Pipeline trigger code cannot be empty.")
        }
    }

    /// - trigger type cannot be none
    /// - for topic is synonym, only [PipelineTriggerType::Insert] is allowed,
    /// - for topic is raw, only [PipelineTriggerType::Insert] is allowed.
    fn check_trigger_type(
        trigger_data: &PipelineTriggerData,
        topic_schema: &TopicSchema,
    ) -> StdR<PipelineTriggerType> {
        match &trigger_data.trigger_type {
            Some(trigger_type) => match trigger_type {
                PipelineTriggerType::Insert => Ok(PipelineTriggerType::Insert),
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
                        Ok(other.clone())
                    }
                }
            },
            _ => PipelineKernelErrorCode::TriggerTypeMissed.msg("Pipeline trigger type is missed."),
        }
    }

    /// create a new trace id when trace id is not given
    fn check_trace_id(trigger_data: &PipelineTriggerData) -> StdR<PipelineTriggerTraceId> {
        if let Some(trace_id) = &trigger_data.trace_id {
            Ok(trace_id.clone())
        } else {
            Ok(IdGen::next_id()?.to_string())
        }
    }

    /// topic data must be presented.
    fn check_data(trigger_data: PipelineTriggerData) -> StdR<TopicData> {
        if let Some(data) = trigger_data.data {
            Ok(data)
        } else {
            PipelineKernelErrorCode::TriggerDataMissed.msg("Pipeline trigger data cannot be empty.")
        }
    }

    fn create_execute_principal(principal: &Principal, tenant_id: TenantId) -> Principal {
        if principal.is_super_admin() {
            // switch to given tenant and fake as admin role
            principal.switch_tenant(tenant_id, UserRole::Admin)
        } else {
            // use current principal
            principal.clone()
        }
    }

    fn prepare(
        principal: &Principal,
        trigger_data: PipelineTriggerData,
    ) -> StdR<PipelineExecuteInput> {
        let tenant_id = Self::check_tenant(principal, &trigger_data);
        let topic_code = Self::check_trigger_code(&trigger_data);
        let (topic_schema, trigger_type) = match (&tenant_id, &topic_code) {
            (Ok(tenant_id), Ok(topic_code)) => {
                match TopicService::schema()?.by_code(topic_code, tenant_id) {
                    Ok(topic_schema) => {
                        let trigger_type = Self::check_trigger_type(&trigger_data, &topic_schema);
                        (Some(Ok(topic_schema)), Some(trigger_type))
                    }
                    err => (Some(err), None),
                }
            }
            _ => (None, None),
        };
        let trace_id = Self::check_trace_id(&trigger_data);
        let topic_data = Self::check_data(trigger_data);
        match (
            tenant_id,
            topic_code,
            topic_schema,
            trigger_type,
            topic_data,
            trace_id,
        ) {
            // check passed
            (
                Ok(tenant_id),
                Ok(_),
                Some(Ok(topic_schema)),
                Some(Ok(trigger_type)),
                Ok(topic_data),
                Ok(trace_id),
            ) => Ok(PipelineExecuteInput::new(
                Arc::new(Self::create_execute_principal(principal, tenant_id)),
                topic_schema,
                Arc::new(trigger_type),
                topic_data,
                Arc::new(trace_id),
            )),
            // check failed, collect all errors, and raise
            (tenant_id, topic_code, topic_schema, trigger_type, topic_data, trace_id) => {
                let mut errors = vec![];
                if let Err(tenant_id_err) = tenant_id {
                    errors.push(tenant_id_err);
                }
                if let Err(topic_code_err) = topic_code {
                    errors.push(topic_code_err);
                }
                if let Some(Err(topic_schema_err)) = topic_schema {
                    errors.push(topic_schema_err);
                }
                if let Some(Err(trigger_type_err)) = trigger_type {
                    errors.push(trigger_type_err);
                }
                if let Err(topic_data_err) = topic_data {
                    errors.push(topic_data_err);
                }
                if let Err(trace_id_err) = trace_id {
                    errors.push(trace_id_err);
                }
                // any error occurred
                Err(errors.accumulate().unwrap_err())
            }
        }
    }

    pub fn with(principal: &Principal, trigger_data: PipelineTriggerData) -> StdR<Self> {
        Ok(Self {
            input: Self::prepare(principal, trigger_data)?,
        })
    }

    pub fn execute(self) -> StdR<TopicDataId> {
        let request = PipelinePreExecute::pre_execute(self.input, None)?;
        PipelineExecutor::execute(request)
    }

    pub async fn execute_async(self) -> StdR<TopicDataId> {
        let request = PipelinePreExecute::pre_execute(self.input, None)?;
        PipelineExecutor::execute_async(request).await
    }

    pub fn execute_single(self, pipeline_id: &PipelineId) -> StdR<TopicDataId> {
        let request = PipelinePreExecute::pre_execute(self.input, Some(&pipeline_id))?;
        PipelineExecutor::execute(request)
    }

    pub async fn execute_single_async(self, pipeline_id: &PipelineId) -> StdR<TopicDataId> {
        let request = PipelinePreExecute::pre_execute(self.input, Some(&pipeline_id))?;
        PipelineExecutor::execute_async(request).await
    }
}
