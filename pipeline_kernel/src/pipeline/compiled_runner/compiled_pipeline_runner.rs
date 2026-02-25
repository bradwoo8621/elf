use crate::{
    CompiledPipeline, CompiledStageRunner, InMemoryData, MonitorLogHelper, PipelineExecutionTask,
    StageRunResult,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{StdErr, StdR};
use elf_model::{
    MonitorLogStatus, PipelineMonitorLog, PipelineTopicData, PipelineTriggerTraceId,
    StageMonitorLog, TopicDataId,
};
use elf_runtime_model_kernel::IdGen;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledPipelineRunner<'a> {
    topic_data_id: &'a TopicDataId,

    compiled_pipeline: Arc<CompiledPipeline>,
    principal: Arc<Principal>,
    trace_id: &'a PipelineTriggerTraceId,
    async_monitor_log: bool,

    start_time: NaiveDateTime,
}

impl<'a> CompiledPipelineRunner<'a> {
    pub async fn run(
        in_memory_data: InMemoryData,
        topic_data_id: &'a TopicDataId,
        compiled_pipeline: Arc<CompiledPipeline>,
        principal: Arc<Principal>,
        trace_id: &'a PipelineTriggerTraceId,
        async_monitor_log: bool,
    ) -> Option<Vec<PipelineExecutionTask>> {
        Self {
            topic_data_id,
            compiled_pipeline,
            principal,
            trace_id,
            async_monitor_log,

            start_time: Utc::now().naive_utc(),
        }
        .do_run(in_memory_data)
        .await
    }

    fn create_previous_data_for_log(
        &self,
        in_memory_data: &InMemoryData,
    ) -> Option<PipelineTopicData> {
        match in_memory_data.get_previous_data_opt() {
            Some(data) => Some(MonitorLogHelper::convert_to_log_data_map(data)),
            None => None,
        }
    }

    fn create_current_data_for_log(
        &self,
        in_memory_data: &InMemoryData,
    ) -> Option<PipelineTopicData> {
        match in_memory_data.get_current_data_opt() {
            Some(data) => Some(MonitorLogHelper::convert_to_log_data_map(data)),
            None => None,
        }
    }

    fn check_prerequisite(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.compiled_pipeline.conditional().is_true(in_memory_data)
    }

    fn create_monitor_log(
        &self,
        in_memory_data: &InMemoryData,
        prerequisite: bool,
        // the bool is all stages are run or not
        stage_logs: Option<(Vec<StageMonitorLog>, bool)>,
        error: Option<StdErr>,
    ) -> StdR<PipelineMonitorLog> {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        let (stage_logs, all_stage_accomplished) =
            if let Some((stage_logs, all_stage_accomplished)) = stage_logs {
                (Some(stage_logs), all_stage_accomplished)
            } else {
                (None, true)
            };
        let status = if !all_stage_accomplished || error.is_some() {
            Some(MonitorLogStatus::ERROR)
        } else {
            Some(MonitorLogStatus::DONE)
        };

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
            status,
            start_time: Some(self.start_time),
            spent_in_mills: Some(spent_in_mills),
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            data_id: Some(self.topic_data_id.clone()),
            old_value: self.create_previous_data_for_log(in_memory_data),
            new_value: self.create_current_data_for_log(in_memory_data),
            stages: stage_logs,
        })
    }

    async fn save_monitor_log(
        &self,
        in_memory_data: &InMemoryData,
        prerequisite: bool,
        // the bool is all stages are run or not
        stage_logs: Option<(Vec<StageMonitorLog>, bool)>,
        error: Option<StdErr>,
    ) {
        let _log = self.create_monitor_log(in_memory_data, prerequisite, stage_logs, error);
        let _async_monitor_log = self.async_monitor_log;
        todo!("implement save_monitor_log for CompiledPipelineRunner")
    }

    async fn do_run(self, mut in_memory_data: InMemoryData) -> Option<Vec<PipelineExecutionTask>> {
        match self.check_prerequisite(&mut in_memory_data) {
            Ok(true) => {
                let mut all_stage_accomplished: bool = true;
                let mut stage_logs = vec![];
                let mut created_tasks = vec![];
                for stage in self.compiled_pipeline.stages().iter() {
                    let StageRunResult {
                        created_tasks: created_tasks_by_stage,
                        log,
                    } = CompiledStageRunner::run(
                        &mut in_memory_data,
                        self.compiled_pipeline.clone(),
                        stage.clone(),
                        self.principal.clone(),
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
                        all_stage_accomplished = false;
                        break;
                    }
                }

                self.save_monitor_log(
                    &in_memory_data,
                    true,
                    Some((stage_logs, all_stage_accomplished)),
                    None,
                )
                .await;
                Some(created_tasks)
            }
            Ok(false) => {
                self.save_monitor_log(&in_memory_data, false, None, None)
                    .await;
                None
            }
            Err(error) => {
                self.save_monitor_log(&in_memory_data, true, None, Some(error))
                    .await;
                None
            }
        }
    }
}
