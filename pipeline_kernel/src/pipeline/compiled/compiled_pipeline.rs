use crate::{
    CompiledConditional, CompiledStage, InternalPipelineExecutable, PipelineExecutable,
    PipelineExecutionTask,
};
use chrono::Utc;
use elf_base::StdR;
use elf_model::{MonitorLogStatus, PipelineMonitorLog};
use elf_runtime_model_kernel::{IdGen, PipelineSchema, TopicSchema};
use std::ops::Deref;
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

    async fn do_execute_stages(
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

    pub async fn execute_async(
        &self,
        executable: PipelineExecutable,
    ) -> StdR<Option<Vec<PipelineExecutionTask>>> {
        let start_time = Utc::now().naive_utc();
        let log = PipelineMonitorLog {
            uid: Some(IdGen::next_id()?.to_string()),
            trace_id: Some(executable.trace_id.deref().clone()),
            pipeline_id: Some(self.pipeline.pipeline_id().deref().clone()),
            topic_id: Some(self.topic.topic_id().deref().clone()),
            prerequisite_defined_as: self.conditional.defined_as(),
            status: Some(MonitorLogStatus::DONE),
            start_time: Some(start_time),
            // will set later
            spent_in_mills: Some(0),
            // will set later if any error raised
            error: None,
            // will set after prerequisite checked
            prerequisite: None,
            data_id: Some(executable.topic_data_id.deref().clone()),
            old_value: None,
            new_value: None,
            // will initialize after stage starts
            stages: None,
        };

        let mut executable = InternalPipelineExecutable::create(executable, log);
        let created_tasks = if self.conditional.is_true(executable.in_memory_data())? {
            executable.monitor_log().prerequisite = Some(true);
            self.do_execute_stages(&executable).await?
        } else {
            // skip the execution because doesn't meet the prerequisite
            executable.monitor_log().prerequisite = Some(false);
            None
        };

        executable.monitor_log().spent_in_mills =
            Some((Utc::now().timestamp() - start_time.and_utc().timestamp()) as u32);
        // TODO save monitor log

        Ok(created_tasks)
    }
}
