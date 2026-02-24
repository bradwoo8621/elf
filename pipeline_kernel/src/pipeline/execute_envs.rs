use elf_base::{EnvConfig, ErrorCode, StdErrCode, VoidR};
use std::sync::OnceLock;

pub struct PipelineExecuteEnvs {
    parallel_actions_in_loop_unit: bool,
}

static PIPELINE_EXECUTE_ENVS: OnceLock<PipelineExecuteEnvs> = OnceLock::new();

impl PipelineExecuteEnvs {
    fn init_default() -> Self {
        Self {
            parallel_actions_in_loop_unit: false,
        }
    }

    /// initialize pipeline execute environments by given environment
    /// TIP call it at system startup
    pub fn init(envs: &EnvConfig) -> VoidR {
        let parallel_actions_in_loop_unit = envs
            .get_bool("PIPELINE_PARALLEL_ACTIONS_IN_LOOP_UNIT")?
            .unwrap_or(false);

        let envs = PipelineExecuteEnvs {
            parallel_actions_in_loop_unit,
        };

        PIPELINE_EXECUTE_ENVS.set(envs).or_else(|_| {
            StdErrCode::EnvInit.msg("Failed to initialize pipeline execution environment.")
        })
    }

    pub fn use_parallel_actions_in_loop_unit() -> bool {
        PIPELINE_EXECUTE_ENVS
            .get_or_init(Self::init_default)
            .parallel_actions_in_loop_unit
    }
}
