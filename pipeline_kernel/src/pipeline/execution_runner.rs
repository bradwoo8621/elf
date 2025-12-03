use crate::pipeline::compile_service::PipelineCompileService;
use crate::{PipelineExecutable, PipelineExecution};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId};

pub struct PipelineExecutionRunner {}

impl PipelineExecutionRunner {
    fn find_pipeline_compile_service(tenant_id: &TenantId) -> StdR<Arc<PipelineCompileService>> {
        Ok(PipelineCompileService::with(tenant_id)?)
    }

    pub async fn run(execution: PipelineExecution) -> StdR<Option<Vec<PipelineExecution>>> {
        let compiled_pipeline =
            Self::find_pipeline_compile_service(&execution.principal.tenant_id)?
                .compile(execution.pipeline_schema)?;

        let topic_trigger = execution.topic_trigger;

        compiled_pipeline
            .execute(PipelineExecutable(
                topic_trigger.internal_data_id.clone(),
                topic_trigger.previous.clone(),
                topic_trigger.current.clone(),
                execution.principal,
                execution.trace_id,
                execution.execution_log_monitor,
            ))
            .await
    }
}
