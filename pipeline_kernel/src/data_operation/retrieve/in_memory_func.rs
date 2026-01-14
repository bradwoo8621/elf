use crate::{ArcTopicDataValue, FuncDataPath, InMemoryData, PipelineKernelErrorCode};
use chrono::NaiveDateTime;
use elf_base::{ErrorCode, StdR};
use elf_model::VariablePredefineFunctions;
use std::sync::Arc;

impl FuncDataPath {
    pub fn get_value(&self, in_memory_data: &InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        match self.func() {
            VariablePredefineFunctions::FromPreviousTriggerData => Ok(Arc::new(
                ArcTopicDataValue::Map(in_memory_data.get_previous_data()?.clone()),
            )),
            VariablePredefineFunctions::FromCurrentTriggerData => Ok(Arc::new(
                ArcTopicDataValue::Map(in_memory_data.get_current_data()?.clone()),
            )),
            VariablePredefineFunctions::Now => {
                Ok(Arc::new(ArcTopicDataValue::DateTime(NaiveDateTime::new())))
            }
        }
    }

    pub fn get_value_of(
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

        Ok(Arc::new(ArcTopicDataValue::None))
    }
}
