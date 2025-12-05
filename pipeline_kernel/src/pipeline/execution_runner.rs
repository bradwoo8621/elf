use crate::{PipelineCompilationProvider, PipelineExecutable, PipelineExecution};
use watchmen_model::StdR;
use watchmen_runtime_model_kernel::PipelineService;

pub struct PipelineExecutionRunner {}

impl PipelineExecutionRunner {
    pub async fn run(execution: PipelineExecution) -> StdR<Option<Vec<PipelineExecution>>> {
        let compiled_pipeline = PipelineService::compilation()?
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
