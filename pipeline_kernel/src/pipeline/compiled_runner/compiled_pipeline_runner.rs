use crate::{
    CompiledPipeline, CompiledStageRunner, InMemoryData, PipelineExecutionTask, StageRunResult,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{
    MonitorLogStatus, NotKnownYetDataStruct, PipelineMonitorLog, PipelineTriggerTraceId,
    StageMonitorLog, TopicDataId,
};
use elf_runtime_model_kernel::IdGen;
use std::ops::Deref;

pub struct CompiledPipelineRunner<'a> {
    in_memory_data: InMemoryData,
    topic_data_id: &'a TopicDataId,

    compiled_pipeline: &'a CompiledPipeline,
    principal: &'a Principal,
    trace_id: &'a PipelineTriggerTraceId,
    async_monitor_log: bool,

    start_time: NaiveDateTime,
}

impl<'a> CompiledPipelineRunner<'a> {
    pub async fn run(
        in_memory_data: InMemoryData,
        topic_data_id: &'a TopicDataId,
        compiled_pipeline: &'a CompiledPipeline,
        principal: &'a Principal,
        trace_id: &'a PipelineTriggerTraceId,
        async_monitor_log: bool,
    ) -> Option<Vec<PipelineExecutionTask>> {
        Self {
            in_memory_data,
            topic_data_id,
            compiled_pipeline,
            principal,
            trace_id,
            async_monitor_log,

            start_time: Utc::now().naive_utc(),
        }
        .do_run()
        .await
    }

    fn create_previous_data_for_log(&self) -> Option<NotKnownYetDataStruct> {
        match self.in_memory_data.get_previous_data_opt() {
            Some(_data) => {
                todo!("implement create_previous_data_for_log for CompiledPipelineRunner")
            }
            None => None,
        }
    }

    fn create_current_data_for_log(&self) -> Option<NotKnownYetDataStruct> {
        match self.in_memory_data.get_current_data_opt() {
            Some(_data) => {
                todo!("implement create_current_data_for_log for CompiledPipelineRunner")
            }
            None => None,
        }
    }

    fn check_prerequisite(&mut self) -> StdR<bool> {
        self.compiled_pipeline
            .conditional()
            .is_true(&mut self.in_memory_data)
    }

    fn create_monitor_log(
        &self,
        prerequisite: bool,
        stage_logs: Option<Vec<StageMonitorLog>>,
        error: Option<StdErr>,
    ) -> StdR<PipelineMonitorLog> {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        Ok(PipelineMonitorLog {
            uid: Some(IdGen::next_id()?.to_string()),
            trace_id: Some(self.trace_id.clone()),
            pipeline_id: Some(
                self.compiled_pipeline
                    .pipeline_schema()
                    .pipeline_id()
                    .deref()
                    .clone(),
            ),
            topic_id: Some(
                self.compiled_pipeline
                    .topic_schema()
                    .topic_id()
                    .deref()
                    .clone(),
            ),
            prerequisite_defined_as: self.compiled_pipeline.conditional().defined_as(),
            status: Some(match &error {
                Some(_) => MonitorLogStatus::DONE,
                None => MonitorLogStatus::ERROR,
            }),
            start_time: Some(self.start_time),
            // will set later
            spent_in_mills: Some(spent_in_mills),
            // will set later if any error raised
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            data_id: Some(self.topic_data_id.clone()),
            old_value: self.create_previous_data_for_log(),
            new_value: self.create_current_data_for_log(),
            // will initialize after stage starts
            stages: stage_logs,
        })
    }

    async fn save_monitor_log(
        &self,
        prerequisite: bool,
        stage_logs: Option<Vec<StageMonitorLog>>,
        error: Option<StdErr>,
    ) {
        let _log = self.create_monitor_log(prerequisite, stage_logs, error);
        todo!("implement save_monitor_log for CompiledPipelineRunner")
    }

    async fn do_run(mut self) -> Option<Vec<PipelineExecutionTask>> {
        match self.check_prerequisite() {
            Ok(true) => {
                let mut all_stage_run: bool = true;
                let mut stage_logs = vec![];
                let mut created_tasks = vec![];
                for stage in self.compiled_pipeline.stages().iter() {
                    let StageRunResult {
                        created_tasks: created_tasks_by_stage,
                        log,
                    } = CompiledStageRunner::run(
                        &mut self.in_memory_data,
                        self.compiled_pipeline,
                        stage,
                        self.principal,
                    )
                    .await;

                    // push created tasks by stage into created tasks
                    if let Some(created_tasks_by_stage) = created_tasks_by_stage {
                        created_tasks.extend(created_tasks_by_stage);
                    }
                    // check there is any error occurred in stage running
                    let has_error = match log.status {
                        Some(MonitorLogStatus::ERROR) => true,
                        _ => false,
                    };
                    stage_logs.push(log);
                    if has_error {
                        all_stage_run = false;
                        break;
                    }
                }

                self.save_monitor_log(all_stage_run, Some(stage_logs), None)
                    .await;
                Some(created_tasks)
            }
            Ok(false) => {
                self.save_monitor_log(false, None, None).await;
                None
            }
            Err(error) => {
                self.save_monitor_log(true, None, Some(error)).await;
                None
            }
        }
    }
}
