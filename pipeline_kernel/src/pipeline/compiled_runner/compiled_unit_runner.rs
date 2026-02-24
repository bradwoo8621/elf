use crate::{
    ArcTopicDataValue, CompiledPipeline, CompiledStage, CompiledUnit, InMemoryData,
    MonitorLogHelper, PipelineExecuteEnvs, PipelineExecutionTask, PipelineKernelErrorCode,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{ErrorCode, StdErr};
use elf_model::{ActionMonitorLog, MonitorLogStatus, UnitMonitorLog};
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledUnitRunner {
    compiled_pipeline: Arc<CompiledPipeline>,
    compiled_stage: Arc<CompiledStage>,
    compiled_unit: Arc<CompiledUnit>,
    principal: Arc<Principal>,

    start_time: NaiveDateTime,
}

pub struct UnitRunResult {
    pub created_tasks: Option<Vec<PipelineExecutionTask>>,
    pub log: UnitMonitorLog,
}

impl CompiledUnitRunner {
    pub async fn run(
        in_memory_data: &mut InMemoryData,
        compiled_pipeline: Arc<CompiledPipeline>,
        compiled_stage: Arc<CompiledStage>,
        compiled_unit: Arc<CompiledUnit>,
        principal: Arc<Principal>,
    ) -> Vec<UnitRunResult> {
        Self {
            compiled_pipeline,
            compiled_stage,
            compiled_unit,
            principal,

            start_time: Utc::now().naive_utc(),
        }
        .do_run(in_memory_data)
        .await
    }

    fn create_monitor_log(
        &self,
        prerequisite: bool,
        loop_variable_value: Option<Arc<ArcTopicDataValue>>,
        action_logs: Option<Vec<ActionMonitorLog>>,
        error: Option<StdErr>,
    ) -> UnitMonitorLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        UnitMonitorLog {
            unit_id: Some(self.compiled_unit.stage().stage_id.deref().clone()),
            name: Some(self.compiled_unit.stage().name.deref().clone()),
            loop_variable_name: self.compiled_unit.loop_variable_name().clone(),
            prerequisite_defined_as: self.compiled_unit.conditional().defined_as(),
            status: Some(match &error {
                Some(_) => MonitorLogStatus::DONE,
                None => MonitorLogStatus::ERROR,
            }),
            start_time: Some(self.start_time),
            spent_in_mills: Some(spent_in_mills),
            error: error.map(|e| format!("{}", e)),
            prerequisite: Some(prerequisite),
            loop_variable_value: if let Some(value) = loop_variable_value {
                if let Some(converted_value) = MonitorLogHelper::convert_to_log_value(value.deref())
                {
                    Some(converted_value)
                } else {
                    None
                }
            } else {
                None
            },
            actions: action_logs,
        }
    }

    async fn do_run_no_loop(&self, in_memory_data: &mut InMemoryData) -> UnitRunResult {
        todo!("implement do_run_no_loop for CompiledUnitRunner")
    }

    async fn do_run_for_loop(&self, in_memory_data: InMemoryData) -> UnitRunResult {
        todo!("implement do_run_for_loop for CompiledUnitRunner")
    }

    fn get_loop_variable(
        &self,
        in_memory_data: &InMemoryData,
        loop_variable_name: &String,
    ) -> Option<Arc<ArcTopicDataValue>> {
        in_memory_data
            .get_variables()
            .get(loop_variable_name)
            .map(|v| v.clone())
    }

    fn clone_runner_for_loop(&self) -> Self {
        Self {
            compiled_pipeline: self.compiled_pipeline.clone(),
            compiled_stage: self.compiled_stage.clone(),
            compiled_unit: self.compiled_unit.clone(),
            principal: self.principal.clone(),
            start_time: Utc::now().naive_utc(),
        }
    }

    fn clone_in_memory_data_for_loop(
        &self,
        in_memory_data: &InMemoryData,
        loop_variable_name: &String,
        loop_variable_value: &ArcTopicDataValue,
    ) -> InMemoryData {
        in_memory_data.fork_with(loop_variable_name, loop_variable_value.clone())
    }

    async fn loop_with_variable(
        &self,
        in_memory_data: &mut InMemoryData,
        loop_variable_name: &String,
        vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
    ) -> Vec<UnitRunResult> {
        let mut results = vec![];
        match (
            PipelineExecuteEnvs::use_parallel_actions_in_loop_unit(),
            PipelineExecuteEnvs::loop_parallel_thread_pool_size(),
        ) {
            (true, _) => {
                // parallel, doesn't matter the pool size, all tasks handle over to Tokio,
                // because almost all actions are I/O-intensive
                let mut handles = vec![];
                for element in vec.iter() {
                    let runner = self.clone_runner_for_loop();
                    let in_memory_data = self.clone_in_memory_data_for_loop(
                        in_memory_data,
                        loop_variable_name,
                        element,
                    );
                    let handle =
                        tokio::spawn(async move { runner.do_run_for_loop(in_memory_data).await });
                    handles.push(handle);
                }
                for handle in handles {
                    match handle.await {
                        Ok(result) => {
                            results.push(result);
                        }
                        Err(e) => {
                            // TODO how to handle join error?
                            eprintln!("Task panicked: {:?}", e);
                        }
                    }
                }
            }
            (false, _) => {
                // no parallel
                for element in vec.iter() {
                    let runner = self.clone_runner_for_loop();
                    let mut in_memory_data = self.clone_in_memory_data_for_loop(
                        in_memory_data,
                        loop_variable_name,
                        element,
                    );
                    results.push(runner.do_run_no_loop(&mut in_memory_data).await);
                }
            }
        }
        results
    }

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> Vec<UnitRunResult> {
        if self.compiled_unit.has_loop() {
            if let Some(loop_variable_name) = self.compiled_unit.loop_variable_name() {
                let loop_variable_value =
                    self.get_loop_variable(in_memory_data, loop_variable_name);
                match loop_variable_value {
                    None => {
                        vec![UnitRunResult {
                            created_tasks: None,
                            log: self.create_monitor_log(true, None, None, None),
                        }]
                    }
                    Some(value) => {
                        match value.deref() {
                            ArcTopicDataValue::None => {
                                vec![UnitRunResult {
                                    created_tasks: None,
                                    log: self.create_monitor_log(true, None, None, None),
                                }]
                            }
                            ArcTopicDataValue::Vec(vec) => {
                                if vec.is_empty() {
                                    vec![UnitRunResult {
                                        created_tasks: None,
                                        log: self.create_monitor_log(true, Some(value), None, None),
                                    }]
                                } else {
                                    self.loop_with_variable(in_memory_data, loop_variable_name, vec)
                                        .await
                                }
                            }
                            other_value => {
                                // only vec and none are supported, raise error
                                vec![UnitRunResult {
                                    created_tasks: None,
                                    log: self.create_monitor_log(
                                        true,
                                        Some(value.clone()),
                                        None,
                                        Some(PipelineKernelErrorCode::UnitLoopVariableNotAVec.e_msg(format!(
                                            "Loop variable[{}] value of action[action_id={}] must be a list, current is [{}].",
                                            loop_variable_name, self.compiled_unit.unit().unit_id, other_value
                                        ))),
                                    ),
                                }]
                            }
                        }
                    }
                }
            } else {
                // never happen
                vec![UnitRunResult {
                    created_tasks: None,
                    log: self.create_monitor_log(
                        true,
                        None,
                        None,
                        Some(
                            PipelineKernelErrorCode::UnitLoopVariableMissed.e_msg(format!(
                                "Variable name of unit[unit_id={}] is missed.",
                                self.compiled_unit.unit().unit_id
                            )),
                        ),
                    ),
                }]
            }
        } else {
            let result = self.do_run_no_loop(in_memory_data).await;
            vec![result]
        }
    }
}
