use crate::PipelineKernelErrorCode;
use watchmen_auth::Principal;
use watchmen_model::{
	PipelineTriggerData, PipelineTriggerTraceId, StdErr, StdErrorCode, StringUtils, TopicDataId,
	User, UserRole,
};

pub struct PipelineInvoker;

impl PipelineInvoker {
    fn check_trigger_data(trigger_data: &PipelineTriggerData) -> Result<(), StdErr> {
        if trigger_data.data.is_none() {
            StdErr::of(
                PipelineKernelErrorCode::EmptyTriggerData.code(),
                "Trigger data is null.",
            )
        } else {
            Ok(())
        }
    }

    fn ask_execute_principal(
        trigger_data: &PipelineTriggerData,
        principal: Principal,
    ) -> Result<Principal, StdErr> {
        if principal.is_super_admin() {
            if trigger_data.tenant_id.is_blank() {
                return StdErr::of(
                    PipelineKernelErrorCode::TenantIdMissedInTriggerData.code(),
                    "Tenant id is missing in pipeline trigger data when invoke pipeline by super admin.",
                );
            }
            // let tenant: Option<Tenant> =
            // 	TenantService::with(&principal).find_by_id(&trigger_data.tenant_id);
            // if tenant.is_none() {
            // 	return StdErr::of(
            // 		PipelineKernelErrorCode::TenantNotExists.code(),
            // 		format!("Tenant[{:?}] not exists.", &trigger_data.tenant_id),
            // 	);
            // }
            // run by super admin, fake as a tenant admin.
            // user id and name still use current principal's
            Principal::from_user(
                User::new()
                    .tenant_id(trigger_data.tenant_id.clone().unwrap())
                    .user_id(principal.user_id)
                    .name(principal.name)
                    .role(UserRole::Admin),
            )
        } else {
            match &trigger_data.tenant_id {
                Some(tenant_id_of_trigger_data) => {
                    if tenant_id_of_trigger_data.is_not_blank()
                        && *tenant_id_of_trigger_data != principal.tenant_id
                    {
                        StdErr::of(
                            PipelineKernelErrorCode::TenantIdMismatchedWithPrincipal.code(),
                            format!(
                                "Tenant id[{}] does not match principal.",
                                tenant_id_of_trigger_data
                            ),
                        )
                    } else {
                        Ok(principal)
                    }
                }
                _ => Ok(principal),
            }
        }
    }

    pub async fn invoke_async(
        trigger_data: PipelineTriggerData,
        trace_id: PipelineTriggerTraceId,
        principal: Principal,
        asynchronized: bool,
    ) -> Result<TopicDataId, StdErr> {
        Self::check_trigger_data(&trigger_data)?;
        let execute_principal = Self::ask_execute_principal(&trigger_data, principal)?;

        // let schema = TenantService::with(&execute_principal).find_topic_schema(&trigger_data.code, &execute_principal);
        // PipelineTrigger::new(
        // 	trigger_topic_schema: schema,
        // 	trigger_type: trigger_data.triggerType,
        // 	trigger_data: trigger_data.data,
        // 	trace_id,
        // 	principal: execute_principal,
        // 	asynchronized,
        // 	handle_monitor_log: create_monitor_log_pipeline_invoker(trace_id, principal_service),
        // ).invoke().await
        todo!("Not implemented yet")
    }

    pub async fn invoke_sync(
        trigger_data: PipelineTriggerData,
        trace_id: PipelineTriggerTraceId,
        principal: Principal,
    ) -> Result<TopicDataId, StdErr> {
        todo!("Not implemented yet")
    }
}
