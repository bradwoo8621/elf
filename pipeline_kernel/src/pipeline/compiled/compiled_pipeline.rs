use crate::{
    CompiledConditional, CompiledStage, InternalPipelineExecutable, PipelineExecutable,
    PipelineExecutionTask,
};
use elf_base::StdR;
use elf_model::PipelineMonitorLog;
use elf_runtime_model_kernel::{PipelineSchema, TopicSchema};
use std::sync::Arc;

pub struct CompiledPipeline {
    topic: Arc<TopicSchema>,
    pipeline: Arc<PipelineSchema>,
    conditional: CompiledConditional,
    stages: Vec<CompiledStage>,
}

impl CompiledPipeline {
    pub fn compile(
        topic_schema: Arc<TopicSchema>,
        pipeline_schema: Arc<PipelineSchema>,
    ) -> StdR<Self> {
        let pipeline = pipeline_schema.pipeline();
        let compiled_conditional = CompiledConditional::compile(&pipeline.on, &pipeline.tenant_id)?;
        let mut compiled_stages = vec![];
        for stage in pipeline.stages.iter() {
            compiled_stages.push(CompiledStage::compile(stage, &pipeline.tenant_id)?);
        }

        Ok(Self {
            topic: topic_schema,
            pipeline: pipeline_schema,
            conditional: compiled_conditional,
            stages: compiled_stages,
        })
    }

    async fn execute_stages(
        &self,
        executable: &InternalPipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        let mut created_tasks = vec![];

        for stage in self.stages.iter() {
            if let Some(created_tasks_of_stage) = stage.execute(executable).await? {
                created_tasks.extend(created_tasks_of_stage);
            }
        }

        Ok(Some(created_tasks))
    }

    pub async fn execute(
        &self,
        executable: PipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        // TODO to create monitor log
        let log = PipelineMonitorLog {
            uid: None,
            trace_id: None,
            pipeline_id: None,
            topic_id: None,
            data_id: None,
            old_value: None,
            new_value: None,
            stages: None,
        };

        let mut executable = InternalPipelineExecutable::create(executable, log);
        let created_tasks = if self.conditional.is_true(executable.in_memory_data())? {
            self.execute_stages(&executable).await?
        } else {
            // skip the execution because doesn't meet the prerequisite
            None
        };

        // TODO save monitor log

        Ok(created_tasks)
    }
}
