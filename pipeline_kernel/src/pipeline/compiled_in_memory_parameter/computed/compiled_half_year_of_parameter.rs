use crate::{ArcFrom, ArcTopicDataValue, CompiledParameter, InMemoryData, PipelineKernelErrorCode};
use bigdecimal::BigDecimal;
use elf_base::{DateConstValues, DateTimeUtils, ErrorCode, StdR};
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcHalfYearOfParameter;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledHalfYearOfParameter {
    parameter: CompiledParameter,
}

impl CompiledHalfYearOfParameter {
    pub fn compile(param: &Arc<ArcHalfYearOfParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        Ok(CompiledHalfYearOfParameter {
            parameter: CompiledParameter::compile(&param.parameter, tenant_id)?,
        })
    }
}

impl CompiledHalfYearOfParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let result = match self.parameter.value_from(in_memory_data)?.deref() {
            ArcTopicDataValue::None => return Ok(Arc::new(ArcTopicDataValue::None)),
            ArcTopicDataValue::Date(date) => BigDecimal::from(date.half_year()),
            ArcTopicDataValue::DateTime(datetime) => BigDecimal::from(datetime.half_year()),
            ArcTopicDataValue::Str(str) => {
                if str.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::None));
                } else if let Ok(date) = str.to_date_loose() {
                    BigDecimal::from(date.half_year())
                } else {
                    return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                        "Argument value of half-year-of must be a date or datetime, current is [{}].",
                        str
                    ));
                }
            }
            other => {
                return PipelineKernelErrorCode::ComputeParameterNotADate.msg(format!(
                    "Argument value of half-year-of must be a date or datetime, current is [{}].",
                    other
                ));
            }
        };
        Ok(ArcTopicDataValue::arc_from(result))
    }
}
