use crate::{
    ArcTopicDataValue, CompiledAddParameter, CompiledCaseThenParameter,
    CompiledDayOfMonthParameter, CompiledDayOfWeekParameter, CompiledDivideParameter,
    CompiledHalfYearOfParameter, CompiledModulusParameter, CompiledMonthOfParameter,
    CompiledMultiplyParameter, CompiledQuarterOfParameter, CompiledSubtractParameter,
    CompiledWeekOfMonthParameter, CompiledWeekOfYearParameter, CompiledYearOfParameter,
    InMemoryData, PipelineKernelErrorCode,
};
use elf_base::{ErrorCode, StdR};
use elf_model::TenantId;
use elf_runtime_model_kernel::ArcComputedParameter;
use std::ops::Deref;
use std::sync::Arc;

pub enum CompiledComputedParameter {
    Add(CompiledAddParameter),
    Subtract(CompiledSubtractParameter),
    Multiply(CompiledMultiplyParameter),
    Divide(CompiledDivideParameter),
    Modulus(CompiledModulusParameter),
    YearOf(Box<CompiledYearOfParameter>),
    HalfYearOf(Box<CompiledHalfYearOfParameter>),
    QuarterOf(Box<CompiledQuarterOfParameter>),
    MonthOf(Box<CompiledMonthOfParameter>),
    WeekOfYear(Box<CompiledWeekOfYearParameter>),
    WeekOfMonth(Box<CompiledWeekOfMonthParameter>),
    DayOfMonth(Box<CompiledDayOfMonthParameter>),
    DayOfWeek(Box<CompiledDayOfWeekParameter>),
    CaseThen(Box<CompiledCaseThenParameter>),
}

impl CompiledComputedParameter {
    pub fn compile(parameter: &Arc<ArcComputedParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        match parameter.deref() {
            ArcComputedParameter::None(_) => PipelineKernelErrorCode::ComputeParameterTypeMissed
                .msg("Type of compute parameter is missed."),
            ArcComputedParameter::Add(param) => CompiledAddParameter::compile(param, tenant_id)
                .map(|p| CompiledComputedParameter::Add(p)),
            ArcComputedParameter::Subtract(param) => {
                CompiledSubtractParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::Subtract(p))
            }
            ArcComputedParameter::Multiply(param) => {
                CompiledMultiplyParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::Multiply(p))
            }
            ArcComputedParameter::Divide(param) => {
                CompiledDivideParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::Divide(p))
            }
            ArcComputedParameter::Modulus(param) => {
                CompiledModulusParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::Modulus(p))
            }
            ArcComputedParameter::YearOf(param) => {
                CompiledYearOfParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::YearOf(Box::new(p)))
            }
            ArcComputedParameter::HalfYearOf(param) => {
                CompiledHalfYearOfParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::HalfYearOf(Box::new(p)))
            }
            ArcComputedParameter::QuarterOf(param) => {
                CompiledQuarterOfParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::QuarterOf(Box::new(p)))
            }
            ArcComputedParameter::MonthOf(param) => {
                CompiledMonthOfParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::MonthOf(Box::new(p)))
            }
            ArcComputedParameter::WeekOfYear(param) => {
                CompiledWeekOfYearParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::WeekOfYear(Box::new(p)))
            }
            ArcComputedParameter::WeekOfMonth(param) => {
                CompiledWeekOfMonthParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::WeekOfMonth(Box::new(p)))
            }
            ArcComputedParameter::DayOfMonth(param) => {
                CompiledDayOfMonthParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::DayOfMonth(Box::new(p)))
            }
            ArcComputedParameter::DayOfWeek(param) => {
                CompiledDayOfWeekParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::DayOfWeek(Box::new(p)))
            }
            ArcComputedParameter::CaseThen(param) => {
                CompiledCaseThenParameter::compile(param, tenant_id)
                    .map(|p| CompiledComputedParameter::CaseThen(Box::new(p)))
            }
        }
    }
}

impl CompiledComputedParameter {
    pub fn value_from(&self, in_memory_data: &mut InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        match self {
            Self::Add(v) => v.value_from(in_memory_data),
            Self::Subtract(v) => v.value_from(in_memory_data),
            Self::Multiply(v) => v.value_from(in_memory_data),
            Self::Divide(v) => v.value_from(in_memory_data),
            Self::Modulus(v) => v.value_from(in_memory_data),
            Self::YearOf(v) => v.value_from(in_memory_data),
            Self::HalfYearOf(v) => v.value_from(in_memory_data),
            Self::QuarterOf(v) => v.value_from(in_memory_data),
            Self::MonthOf(v) => v.value_from(in_memory_data),
            Self::WeekOfYear(v) => v.value_from(in_memory_data),
            Self::WeekOfMonth(v) => v.value_from(in_memory_data),
            Self::DayOfMonth(v) => v.value_from(in_memory_data),
            Self::DayOfWeek(v) => v.value_from(in_memory_data),
            Self::CaseThen(v) => v.value_from(in_memory_data),
        }
    }
}
