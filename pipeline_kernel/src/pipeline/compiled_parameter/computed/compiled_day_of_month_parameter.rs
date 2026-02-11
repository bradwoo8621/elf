use crate::{ArcFrom, ArcTopicDataValue, CompiledParameter, InMemoryData, PipelineKernelErrorCode};
use bigdecimal::BigDecimal;
use chrono::Datelike;
use elf_base::{DateTimeUtils, ErrorCode, StdR};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcDayOfMonthParameter, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledDayOfMonthParameter {
    parameter: CompiledParameter,
}

impl CompiledDayOfMonthParameter {
    pub fn compile(
        param: &Arc<ArcDayOfMonthParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledDayOfMonthParameter {
            parameter: CompiledParameter::compile(&param.parameter, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledDayOfMonthParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let result = match self.parameter.value_from(in_memory_data)?.deref() {
            ArcTopicDataValue::None => return Ok(Arc::new(ArcTopicDataValue::None)),
            ArcTopicDataValue::Date(date) => BigDecimal::from(date.day()),
            ArcTopicDataValue::DateTime(datetime) => BigDecimal::from(datetime.day()),
            ArcTopicDataValue::Str(str) => {
                if str.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                } else if let Ok(date) = str.to_date_loose() {
                    BigDecimal::from(date.day())
                } else {
                    return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                        "Argument value of day-of-month must be a date or datetime, current is [{}].",
                        str
                    ));
                }
            }
            other => {
                return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                    "Argument value of day-of-month must be a date or datetime, current is [{}].",
                    other
                ));
            }
        };
        Ok(ArcTopicDataValue::arc_from(result))
    }
}
