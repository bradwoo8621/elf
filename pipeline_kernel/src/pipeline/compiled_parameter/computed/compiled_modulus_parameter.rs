use crate::{ArcFrom, ArcTopicDataValue, CompiledParameter, InMemoryData, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, Zero};
use elf_base::{ErrorCode, NumericUtils, StdR};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{ArcModulusParameter, TopicSchema};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct CompiledModulusParameter {
    parameters: Vec<CompiledParameter>,
}

impl CompiledModulusParameter {
    pub fn compile(
        param: &Arc<ArcModulusParameter>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        if param.parameters.is_empty() {
            return PipelineKernelErrorCode::ComputeParameterParameterMissed
                .msg("Parameter of modulus is missed.");
        }

        let mut parameters = vec![];
        for parameter in param.parameters.iter() {
            parameters.push(CompiledParameter::compile(
                parameter,
                topic_schemas,
                tenant_id,
            )?);
        }

        Ok(CompiledModulusParameter { parameters })
    }
}

impl CompiledModulusParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        let mut result = BigDecimal::zero();

        for parameter in &self.parameters {
            let value = parameter.value_from(in_memory_data)?;
            match value.deref() {
                ArcTopicDataValue::Num(num) => {
                    if num.deref().is_zero() {
                        return PipelineKernelErrorCode::ComputeParameterModulusZero
                            .msg("Cannot modulus by zero.");
                    } else {
                        result = result / num.deref()
                    }
                }
                ArcTopicDataValue::Str(str) => {
                    if str.is_empty() {
                        // empty value treated as 0, ignore
                    } else if let Ok(num) = str.to_decimal() {
                        if num.is_zero() {
                            return PipelineKernelErrorCode::ComputeParameterModulusZero
                                .msg("Cannot modulus by zero.");
                        } else {
                            result = result / num;
                        }
                    } else {
                        return PipelineKernelErrorCode::ComputeParameterValueNotSupported.msg(
                            format!(
                                "Argument value of modulus must be a decimal, current is [{}].",
                                str
                            ),
                        );
                    }
                }
                other => {
                    return PipelineKernelErrorCode::ComputeParameterValueNotSupported.msg(
                        format!(
                            "Argument value of modulus must be a decimal, current is [{}].",
                            other
                        ),
                    );
                }
            }
        }

        Ok(ArcTopicDataValue::arc_from(result))
    }
}
