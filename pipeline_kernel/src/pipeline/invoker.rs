use crate::common::PipelineKernelErrorCode;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerData, PipelineTriggerTraceId, StdErr, StdErrorCode, TopicDataId,
};

pub async fn invoke_async(
    trigger_data: PipelineTriggerData,
    trace_id: PipelineTriggerTraceId,
    principal_service: Principal,
    asynchronized: bool,
) -> Result<TopicDataId, StdErr> {
    let trigger_data_map = trigger_data.data;
    if trigger_data_map.is_none() {
        return StdErr::of(
            PipelineKernelErrorCode::EmptyTriggerData.code(),
            "Trigger data is null.",
        );
    }

    todo!("Not implemented yet")
}

pub async fn invoke_sync(
    trigger_data: PipelineTriggerData,
    trace_id: PipelineTriggerTraceId,
    principal_service: Principal,
) -> Result<TopicDataId, StdErr> {
    todo!("Not implemented yet")
}
