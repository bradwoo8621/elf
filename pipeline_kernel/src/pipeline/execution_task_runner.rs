use crate::{
    CompiledPipelineRunner, InMemoryData, PipelineCompilationProvider, PipelineExecutionTask,
};
use elf_base::StdR;
use elf_runtime_model_kernel::PipelineService;
use std::ops::Deref;

pub struct PipelineExecutionTaskRunner;

impl PipelineExecutionTaskRunner {
    pub async fn run_async(
        task: PipelineExecutionTask,
    ) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        let compiled_pipeline =
            PipelineService::compilation()?.compile(task.topic_schema(), task.pipeline_schema())?;

        let topic_data = task.topic_data();
        let created_tasks = CompiledPipelineRunner::run(
            InMemoryData::new(
                topic_data.previous_data().clone(),
                topic_data.current_data().clone(),
            ),
            topic_data.topic_data_id().deref(),
            compiled_pipeline.deref(),
            task.principal().deref(),
            task.trace_id().deref(),
            task.async_monitor_log(),
        )
        .await;

        Ok(created_tasks)
    }
}
