use elf_base::{EnvConfig, ErrorCode, StdErrCode, VoidR};
use std::sync::OnceLock;

pub struct PipelineExecuteEnvs {
    parallel_actions_in_loop_unit: bool,
    loop_parallel_thread_pool_size: usize,
}

static PIPELINE_EXECUTE_ENVS: OnceLock<PipelineExecuteEnvs> = OnceLock::new();

impl PipelineExecuteEnvs {
    fn init_default() -> Self {
        Self {
            parallel_actions_in_loop_unit: false,
            loop_parallel_thread_pool_size: num_cpus::get(),
        }
    }

    /// initialize pipeline execute environments by given environment
    /// TIP call it at system startup
    pub fn init(envs: &EnvConfig) -> VoidR {
        let parallel_actions_in_loop_unit = envs
            .get_bool("PIPELINE_PARALLEL_ACTIONS_IN_LOOP_UNIT")?
            .unwrap_or(false);

        let mut loop_parallel_thread_pool_size = envs
            .get_usize("PIPELINE_LOOP_PARALLEL_THREAD_POOL_SIZE")?
            .unwrap_or(
                // the origin variable name from python version
                envs.get_usize("PIPELINE_PARALLEL_ACTIONS_COUNT")?
                    .unwrap_or(num_cpus::get()),
            );
        loop_parallel_thread_pool_size = if loop_parallel_thread_pool_size == 0 {
            1
        } else {
            loop_parallel_thread_pool_size
        };

        let envs = PipelineExecuteEnvs {
            parallel_actions_in_loop_unit,
            loop_parallel_thread_pool_size,
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

    pub fn loop_parallel_thread_pool_size() -> usize {
        PIPELINE_EXECUTE_ENVS
            .get_or_init(Self::init_default)
            .loop_parallel_thread_pool_size
    }
}
