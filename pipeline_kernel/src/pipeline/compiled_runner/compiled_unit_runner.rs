use crate::{
    ActionRunResult, ArcTopicDataValue, CompiledActionRunner, CompiledPipeline, CompiledStage,
    CompiledUnit, InMemoryData, MonitorLogHelper, PipelineExecuteEnvs, PipelineExecutionTask,
    PipelineKernelErrorCode,
};
use chrono::{NaiveDateTime, Utc};
use elf_auth::Principal;
use elf_base::{ErrorCode, StdErr, StdR};
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

    async fn do_run(self, in_memory_data: &mut InMemoryData) -> Vec<UnitRunResult> {
        match (
            self.compiled_unit.has_loop(),
            self.compiled_unit.loop_variable_name(),
        ) {
            (true, Some(loop_variable_name)) => {
                self.do_run_loop(loop_variable_name, in_memory_data).await
            }
            // never happen
            (true, None) => self.create_monitor_log_of_loop_variable_missed(),
            (false, _) => {
                let result = self.do_run_unit(None, in_memory_data).await;
                vec![result]
            }
        }
    }
}

impl CompiledUnitRunner {
    fn create_monitor_log(
        &self,
        prerequisite: bool,
        loop_variable_value: Option<Arc<ArcTopicDataValue>>,
        // the bool is all actions are run or not
        action_logs: Option<(Vec<ActionMonitorLog>, bool)>,
        error: Option<StdErr>,
    ) -> UnitMonitorLog {
        let spent_in_mills =
            (Utc::now().timestamp() - self.start_time.and_utc().timestamp()) as u32;

        let (action_logs, all_action_accomplished) =
            if let Some((action_logs, all_action_accomplished)) = action_logs {
                (Some(action_logs), all_action_accomplished)
            } else {
                (None, true)
            };
        let status = if !all_action_accomplished || error.is_some() {
            Some(MonitorLogStatus::ERROR)
        } else {
            Some(MonitorLogStatus::DONE)
        };

        UnitMonitorLog {
            unit_id: Some(self.compiled_unit.stage().stage_id.deref().clone()),
            name: Some(self.compiled_unit.stage().name.deref().clone()),
            loop_variable_name: self.compiled_unit.loop_variable_name().clone(),
            prerequisite_defined_as: self.compiled_unit.conditional().defined_as(),
            status,
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

    fn create_monitor_log_of_loop_variable_missed(&self) -> Vec<UnitRunResult> {
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

    fn create_loop_run_monitor_log_of_none(&self) -> Vec<UnitRunResult> {
        vec![UnitRunResult {
            created_tasks: None,
            log: self.create_monitor_log(true, None, None, None),
        }]
    }

    fn create_loop_run_monitor_log_of_empty_vec(
        &self,
        value: Arc<ArcTopicDataValue>,
    ) -> Vec<UnitRunResult> {
        vec![UnitRunResult {
            created_tasks: None,
            log: self.create_monitor_log(true, Some(value), None, None),
        }]
    }

    fn create_loop_run_monitor_log_of_not_none_or_vec(
        &self,
        loop_variable_name: &String,
        value: Arc<ArcTopicDataValue>,
    ) -> Vec<UnitRunResult> {
        vec![UnitRunResult {
            created_tasks: None,
            log: self.create_monitor_log(
                true,
                Some(value.clone()),
                None,
                Some(PipelineKernelErrorCode::UnitLoopVariableNotAVec.e_msg(format!(
                    "Loop variable[{}] value of action[action_id={}] must be a vec, current is [{}].",
                    loop_variable_name, self.compiled_unit.unit().unit_id, value
                ))),
            ),
        }]
    }
}

impl CompiledUnitRunner {
    fn check_prerequisite(&self, in_memory_data: &mut InMemoryData) -> StdR<bool> {
        self.compiled_unit.conditional().is_true(in_memory_data)
    }

    /// run unit
    /// - when unit has loop, loop variable already merged into in-memory data,
    ///   and the [loop_variable_value] still passed in, since monitor log needs it.
    /// - otherwise, the [loop_variable_value] is [None].
    async fn do_run_unit(
        &self,
        loop_variable_value: Option<Arc<ArcTopicDataValue>>,
        in_memory_data: &mut InMemoryData,
    ) -> UnitRunResult {
        match self.check_prerequisite(in_memory_data) {
            Ok(true) => {
                let mut all_action_accomplished: bool = true;
                let mut action_logs = vec![];
                let mut created_tasks = vec![];

                for action in self.compiled_unit.actions().iter() {
                    let ActionRunResult {
                        created_tasks: created_tasks_by_action,
                        log,
                    } = CompiledActionRunner::run(
                        in_memory_data,
                        self.compiled_pipeline.clone(),
                        self.compiled_stage.clone(),
                        self.compiled_unit.clone(),
                        action.clone(),
                        self.principal.clone(),
                    )
                    .await;

                    // push created tasks by stage into created tasks
                    if let Some(created_tasks_by_action) = created_tasks_by_action {
                        created_tasks.extend(created_tasks_by_action);
                    }
                    // check there is any error occurred in stage running
                    let has_error = ActionRunResult::has_error(&log);
                    action_logs.push(log);

                    if has_error {
                        all_action_accomplished = false;
                        break;
                    }
                }

                UnitRunResult {
                    created_tasks: Some(created_tasks),
                    log: self.create_monitor_log(
                        true,
                        loop_variable_value,
                        Some((action_logs, all_action_accomplished)),
                        None,
                    ),
                }
            }
            Ok(false) => UnitRunResult {
                created_tasks: None,
                log: self.create_monitor_log(false, loop_variable_value, None, None),
            },
            Err(error) => UnitRunResult {
                created_tasks: None,
                log: self.create_monitor_log(true, loop_variable_value, None, Some(error)),
            },
        }
    }
}

impl CompiledUnitRunner {
    fn get_loop_variable_value(
        &self,
        in_memory_data: &InMemoryData,
        loop_variable_name: &String,
    ) -> Option<Arc<ArcTopicDataValue>> {
        in_memory_data
            .get_variables()
            .get(loop_variable_name)
            .map(|v| v.clone())
    }

    fn create_runner_for_loop_round(&self) -> Self {
        Self {
            compiled_pipeline: self.compiled_pipeline.clone(),
            compiled_stage: self.compiled_stage.clone(),
            compiled_unit: self.compiled_unit.clone(),
            principal: self.principal.clone(),
            start_time: Utc::now().naive_utc(),
        }
    }

    fn create_in_memory_data_for_loop_round(
        &self,
        in_memory_data: &InMemoryData,
        loop_variable_name: &String,
        loop_variable_value: &ArcTopicDataValue,
    ) -> InMemoryData {
        in_memory_data.fork_with(loop_variable_name, loop_variable_value.clone())
    }

    async fn do_run_loop_with_variable(
        &self,
        in_memory_data: &mut InMemoryData,
        loop_variable_name: &String,
        loop_variable_value_vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
    ) -> Vec<UnitRunResult> {
        if PipelineExecuteEnvs::use_parallel_actions_in_loop_unit() {
            self.do_run_loop_parallel(in_memory_data, loop_variable_name, loop_variable_value_vec)
                .await
        } else {
            self.do_run_loop_sequential(in_memory_data, loop_variable_name, loop_variable_value_vec)
                .await
        }
    }

    async fn do_run_loop_parallel(
        &self,
        in_memory_data: &mut InMemoryData,
        loop_variable_name: &String,
        loop_variable_value_vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
    ) -> Vec<UnitRunResult> {
        let mut results = vec![];

        // parallel, doesn't matter the pool size, all tasks handle over to Tokio,
        // because almost all actions are I/O-intensive
        // to get the thread pool size by PipelineExecuteEnvs::loop_parallel_thread_pool_size()

        let mut handles = vec![];
        for element in loop_variable_value_vec.iter() {
            let runner = self.create_runner_for_loop_round();
            let loop_variable_value = element.clone();
            let mut in_memory_data = self.create_in_memory_data_for_loop_round(
                in_memory_data,
                loop_variable_name,
                element,
            );
            let handle = tokio::spawn(async move {
                runner
                    .do_run_unit(Some(loop_variable_value), &mut in_memory_data)
                    .await
            });
            handles.push((handle, element));
        }
        for (handle, element) in handles {
            match handle.await {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => results.push(UnitRunResult {
                    created_tasks: None,
                    log:
                    self.create_monitor_log(
                        true,
                        Some(element.clone()),
                        None,
                        Some(PipelineKernelErrorCode::ParallelUnitExecute.e_msg(
                            format!(
                                "Unit[id={}] parallel execution by loop variable[{}] failed, cause by {}.",
                                self.compiled_unit.unit().unit_id,
                                self.compiled_unit.loop_variable_name().as_ref().unwrap(),
                                e
                            ),
                        )),
                    ),
                }),
            }
        }

        results
    }

    async fn do_run_loop_sequential(
        &self,
        in_memory_data: &mut InMemoryData,
        loop_variable_name: &String,
        loop_variable_value_vec: &Arc<Vec<Arc<ArcTopicDataValue>>>,
    ) -> Vec<UnitRunResult> {
        let mut results = vec![];

        for element in loop_variable_value_vec.iter() {
            let runner = self.create_runner_for_loop_round();
            let mut in_memory_data = self.create_in_memory_data_for_loop_round(
                in_memory_data,
                loop_variable_name,
                element,
            );
            results.push(
                runner
                    .do_run_unit(Some(element.clone()), &mut in_memory_data)
                    .await,
            );
        }

        results
    }

    async fn do_run_loop(
        &self,
        loop_variable_name: &String,
        in_memory_data: &mut InMemoryData,
    ) -> Vec<UnitRunResult> {
        match self.get_loop_variable_value(in_memory_data, loop_variable_name) {
            None => self.create_loop_run_monitor_log_of_none(),
            Some(value) => {
                match value.deref() {
                    ArcTopicDataValue::None => self.create_loop_run_monitor_log_of_none(),
                    ArcTopicDataValue::Vec(vec) => {
                        if vec.is_empty() {
                            self.create_loop_run_monitor_log_of_empty_vec(value)
                        } else {
                            self.do_run_loop_with_variable(in_memory_data, loop_variable_name, vec)
                                .await
                        }
                    }
                    // only vec and none are supported, raise error
                    _ => self
                        .create_loop_run_monitor_log_of_not_none_or_vec(loop_variable_name, value),
                }
            }
        }
    }
}
