use crate::{
    ArcFrom, ArcTopicDataValue, FuncDataPath, InMemoryData,
    PipelineKernelErrorCode,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Timelike, Utc};
use elf_base::{ErrorCode, StdR};
use elf_model::VariablePredefineFunctions;
use elf_runtime_model_kernel::IdGen;
use std::sync::Arc;

impl FuncDataPath {
    pub fn value_from_memory(&self, in_memory_data: &InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let func = self.func();
        match func {
            VariablePredefineFunctions::NextSeq => {
                if let Some(seq) = BigDecimal::from_u128(IdGen::next_id()?) {
                    Ok(ArcTopicDataValue::arc_from(seq))
                } else {
                    PipelineKernelErrorCode::FailedToGetNextSeq.msg(format!(
                        "Failed to get next sequence[path={}, name={}].",
                        self.full_path(),
                        func
                    ))
                }
            }
            VariablePredefineFunctions::FromPreviousTriggerData => Ok(ArcTopicDataValue::wrap(
                in_memory_data.get_previous_data()?.clone(),
            )),
            VariablePredefineFunctions::FromCurrentTriggerData => Ok(ArcTopicDataValue::wrap(
                in_memory_data.get_current_data()?.clone(),
            )),
            VariablePredefineFunctions::Now => Ok(ArcTopicDataValue::arc_from(
                Utc::now().naive_utc().with_nanosecond(0).unwrap(),
            )),
            _ => {
                // the first parameter of function is context
                let (params, param_count) = if let Some(params) = self.params() {
                    let param_count = params.len();
                    if param_count == 0 {
                        return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                            "Function[path={}, name={}] has no enough parameters, at least a context parameter is required, but nothing is currently provided.",
                            self.full_path(),
                            func
                        ));
                    }
                    (params, param_count)
                } else {
                    return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                        "Function[path={}, name={}] has no enough parameters, at least a context parameter is required, but nothing is currently provided.",
                        self.full_path(),
                        func
                    ));
                };
                let min_param_count = func.min_param_count();
                if param_count < min_param_count + 1 {
                    // the first one is context
                    return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                        "Function[path={}, name={}] has no enough parameters, at least {} are required, but only {} are currently provided.",
                        self.full_path(),
                        func, min_param_count, param_count
                    ));
                }

                // get context
                let param_0 = &params[0];
                let context = param_0.value_from_memory(in_memory_data)?;
                self.get_value(&context, 1, in_memory_data)
            }
        }
    }

    pub fn value_from_source(
        &self,
        source: &Arc<ArcTopicDataValue>,
        in_memory_data: &InMemoryData,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let func = self.func();
        if !func.require_context() {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Function[path={}, name={}] cannot have context.",
                self.full_path(),
                func
            ));
        }

        self.get_value(source, 0, in_memory_data)
    }

    fn get_value(
        &self,
        context: &Arc<ArcTopicDataValue>,
        param_start_index: usize,
        in_memory_data: &InMemoryData,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        todo!("implement get_value for FuncDataPath")
    }
}
