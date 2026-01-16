use crate::InMemoryFuncCall;
use elf_base::{EnvConfig, ErrorCode, StdErrCode, VoidR};
use std::sync::OnceLock;

struct InMemoryCallSetting {
    join_default_use_comma: bool,
}

static IN_MEMORY_FUNC_CALL_SETTING: OnceLock<InMemoryCallSetting> = OnceLock::new();

impl InMemoryFuncCall<'_> {
    fn init_default() -> InMemoryCallSetting {
        InMemoryCallSetting {
            join_default_use_comma: false,
        }
    }

    /// initialize in-memory function call by given environment
    /// TIP call it at system startup
    pub fn init(envs: &EnvConfig) -> VoidR {
        let join_default_use_comma = envs
            .get_bool("FUNC_JOIN_DEFAULT_USE_COMMA")?
            .unwrap_or(false);

        let setting = InMemoryCallSetting {
            join_default_use_comma,
        };

        IN_MEMORY_FUNC_CALL_SETTING
            .set(setting)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize aes key and iv."))
    }

    pub fn is_func_join_default_use_comma() -> bool {
        IN_MEMORY_FUNC_CALL_SETTING
            .get_or_init(Self::init_default)
            .join_default_use_comma
    }
}
