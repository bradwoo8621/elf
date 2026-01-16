use crate::{InMemoryFuncCall, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdErrCode, StdR};
use elf_model::VariablePredefineFunctions;
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

    pub fn str_parse_error<R>(&self, value: impl Display) -> StdR<R> {
        StdErrCode::DecimalParse.msg(format!(
            "Cannot retrieve[key={}, current={}] as str, cause by current value is [{}].",
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

    pub fn param_count_not_enough<R>(
        &self,
        func: &VariablePredefineFunctions,
        count: usize,
    ) -> StdR<R> {
        PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
            "Cannot retrieve[key={}, current={}], cause by no enough parameters, at least {} parameters, currently only {} provided.",
            self.full_path(),
            self.this_path(),
            func.min_param_count(),
            count
        ))
    }

    pub fn param_count_too_many<R>(
        &self,
        func: &VariablePredefineFunctions,
        count: usize,
    ) -> StdR<R> {
        PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
            "Cannot retrieve[key={}, current={}], cause by too many parameters, at most {} parameters, currently {} provided.",
            self.full_path(),
            self.this_path(),
            func.max_param_count().unwrap_or(0),
            count,
        ))
    }

    pub fn param_must_be_str<R>(
        &self,
        func: &VariablePredefineFunctions,
        param_index: usize,
        value: impl Display,
    ) -> StdR<R> {
        PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
            "Cannot retrieve[key={}, current={}], cause by function[{}] parameter[{}] must be a string, current is [{}].",
            self.full_path(),
            self.this_path(),
            func,
            param_index,
            value
        ))
    }

    pub fn param_must_be_num<R>(
        &self,
        func: &VariablePredefineFunctions,
        param_index: usize,
        value: impl Display,
    ) -> StdR<R> {
        PipelineKernelErrorCode::VariableFuncNotSupported.msg(format!(
            "Cannot retrieve[key={}, current={}], cause by function[{}] parameter[{}] must be a number, current is [{}].",
            self.full_path(),
            self.this_path(),
            func,
            param_index,
            value
        ))
    }
}
