use crate::{ArcFrom, ArcTopicDataValue, CompiledParameter, InMemoryData, PipelineKernelErrorCode};
use bigdecimal::BigDecimal;
use elf_base::{DateConstValues, DateTimeUtils, ErrorCode, StdR};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcWeekOfYearParameter, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledWeekOfYearParameter {
    parameter: CompiledParameter,
}

impl CompiledWeekOfYearParameter {
    pub fn compile(
        param: &Arc<ArcWeekOfYearParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        Ok(CompiledWeekOfYearParameter {
            parameter: CompiledParameter::compile(&param.parameter, topic_schemas, tenant_id)?,
        })
    }
}

impl CompiledWeekOfYearParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let result = match self.parameter.value_from(in_memory_data)?.deref() {
            ArcTopicDataValue::None => return Ok(Arc::new(ArcTopicDataValue::None)),
            ArcTopicDataValue::Date(date) => BigDecimal::from(date.week_of_year()),
            ArcTopicDataValue::DateTime(datetime) => BigDecimal::from(datetime.week_of_year()),
            ArcTopicDataValue::Str(str) => {
                if str.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                } else if let Ok(date) = str.to_date_loose() {
                    BigDecimal::from(date.week_of_year())
                } else {
                    return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                        "Argument value of week-of-year must be a date or datetime, current is [{}].",
                        str
                    ));
                }
            }
            other => {
                return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                    "Argument value of week-of-year must be a date or datetime, current is [{}].",
                    other
                ));
            }
        };
        Ok(ArcTopicDataValue::arc_from(result))
    }
}
