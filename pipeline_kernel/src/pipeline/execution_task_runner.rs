use crate::{PipelineCompilationProvider, PipelineExecutable, PipelineExecutionTask};
use elf_base::StdR;
use elf_runtime_model_kernel::PipelineService;

pub struct PipelineExecutionTaskRunner;

impl PipelineExecutionTaskRunner {
    pub async fn run(task: PipelineExecutionTask) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        let compiled_pipeline =
            PipelineService::compilation()?.compile(task.topic_schema(), task.pipeline_schema())?;

        compiled_pipeline
            .execute(PipelineExecutable::new(
                task.topic_data(),
                task.principal(),
                task.trace_id(),
            ))
            .await
    }
}
