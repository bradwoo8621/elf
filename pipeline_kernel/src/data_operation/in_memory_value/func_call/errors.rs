use crate::{InMemoryFuncCall, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdErrCode, StdR};
use std::fmt::Display;

/// for errors
impl InMemoryFuncCall<'_> {
    pub fn context_disallowed<R>(&self) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Function[path={}, name={}] doesn't allow context.",
            self.full_path(),
            self.this_path()
        ))
    }

    pub fn decimal_parse_error<R>(&self, value: impl Display) -> StdR<R> {
        StdErrCode::DecimalParse.msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal, cause by current value is [{}].",
            self.full_path(),
            self.this_path(),
            value
        ))
    }

    pub fn func_not_supported<R>(&self, value: impl Display) -> StdR<R> {
        PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
            "Cannot retrieve[key={}, current={}], caused by function not supports value [{}].",
            self.full_path(),
            self.this_path(),
            value
        ))
    }
}
