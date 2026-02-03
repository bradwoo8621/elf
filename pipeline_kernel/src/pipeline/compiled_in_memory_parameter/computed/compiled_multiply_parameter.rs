use crate::{ArcFrom, ArcTopicDataValue, CompiledParameter, InMemoryData, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, Zero};
use elf_base::{ErrorCode, NumericUtils, StdR};
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcMultiplyParameter;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledMultiplyParameter {
    parameters: Vec<CompiledParameter>,
}

impl CompiledMultiplyParameter {
    pub fn compile(param: &Arc<ArcMultiplyParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        if param.parameters.is_empty() {
            return PipelineKernelErrorCode::ComputeParameterParameterMissed
                .msg("Parameter of multiply is missed.");
        }

        let mut parameters = vec![];
        for parameter in param.parameters.iter() {
            parameters.push(CompiledParameter::compile(parameter, tenant_id)?);
        }

        Ok(CompiledMultiplyParameter { parameters })
    }
}

impl CompiledMultiplyParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let mut result = BigDecimal::zero();

        for parameter in &self.parameters {
            let value = parameter.value_from(in_memory_data)?;
            match value.deref() {
                ArcTopicDataValue::Num(num) => result = result * num.deref(),
                ArcTopicDataValue::Str(str) => {
                    if str.is_empty() {
                        // empty value treated as 0, ignore
                    } else if let Ok(num) = str.to_decimal() {
                        result = result * num;
                    } else {
                        return PipelineKernelErrorCode::ComputeParameterValueNotSupported.msg(
                            format!("Argument value of multiply must be a decimal, current is [{}].", str),
                        );
                    }
                }
                other => {
                    return PipelineKernelErrorCode::ComputeParameterValueNotSupported.msg(
                        format!(
                            "Argument value of multiply must be a decimal, current is [{}].",
                            other
                        ),
                    );
                }
            }
        }

        Ok(ArcTopicDataValue::arc_from(result))
    }
}
