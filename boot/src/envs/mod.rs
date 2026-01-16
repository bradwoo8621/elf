use elf_base::{DateTimeFormatterInitializer, EnvConfig, VoidR};
use elf_pipeline_kernel::InMemoryFuncCall;
use elf_runtime_model_kernel::AesCrypto;

pub struct EnvsBoot;

impl EnvsBoot {
    pub fn init(env_config: &EnvConfig) -> VoidR {
        DateTimeFormatterInitializer::init(env_config)?;
        AesCrypto::init(env_config)?;
        InMemoryFuncCall::init(env_config)?;

        Ok(())
    }
}
