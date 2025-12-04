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
                .compile(execution.topic_schema, execution.pipeline_schema)?;

        compiled_pipeline
            .execute(PipelineExecutable::new(
                execution.topic_trigger,
                execution.principal,
                execution.trace_id,
                execution.execution_log_monitor,
            ))
            .await
    }
}
