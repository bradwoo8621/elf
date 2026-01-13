use crate::{ArcTopicDataMap, ArcTopicDataValue, FuncDataPath, PipelineKernelErrorCode};
use bigdecimal::{BigDecimal, FromPrimitive};
use elf_base::{ErrorCode, StdErr, StdErrCode, StdR};
use elf_model::VariablePredefineFunctions;
use elf_runtime_model_kernel::IdGen;
use std::sync::Arc;

pub struct VariablePredefineFunctionCaller<'a> {
    root: &'a ArcTopicDataMap,
    full_path: &'a String,
    segment: &'a FuncDataPath,
}

impl<'a> VariablePredefineFunctionCaller<'a> {
    pub fn prepare(
        root: &'a ArcTopicDataMap,
        full_path: &'a String,
        segment: &'a FuncDataPath,
    ) -> Self {
        VariablePredefineFunctionCaller {
            root,
            full_path,
            segment,
        }
    }

    fn decimal_parse_error<R>(&self) -> StdR<R> {
        StdErrCode::DecimalParse.msg(format!(
            "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
            self.full_path, self.segment.path, self.root
        ))
    }

    fn function_not_supported<R>(&self) -> StdR<R> {
        Err(self.err_function_not_supported())
    }

    fn err_function_not_supported(&self) -> StdErr {
        PipelineKernelErrorCode::VariableFuncNotSupported.err_with_msg(format!(
            "Cannot retrieve[key={}, current={}] from [{:?}], caused by function not supported.",
            self.full_path, self.segment.path, self.root
        ))
    }

    pub fn value_of(&self, value: &Arc<ArcTopicDataValue>) -> StdR<Arc<ArcTopicDataValue>> {
        let decimal_parse_err = || self.decimal_parse_error();
        let not_support = || self.function_not_supported();
        let not_support_e = || self.err_function_not_supported();

        match self.segment.func {
            VariablePredefineFunctions::NextSeq => IdGen::next_id().map(|v| {
                Arc::new(ArcTopicDataValue::Num(Arc::new(
                    BigDecimal::from_u128(v).unwrap(),
                )))
            }),
            VariablePredefineFunctions::Count => {
                value.count_of_vec_or_map(decimal_parse_err, not_support)
            }
            VariablePredefineFunctions::Length | VariablePredefineFunctions::Len => {
                value.length_of_str_or_num(decimal_parse_err, not_support)
            }
            VariablePredefineFunctions::Slice | VariablePredefineFunctions::Substr => {
                todo!("variable predefine function[&slice], [&substr]")
            }
            VariablePredefineFunctions::Find | VariablePredefineFunctions::Index => {
                todo!("variable predefine function[&find], [&index]")
            }
            VariablePredefineFunctions::StartsWith | VariablePredefineFunctions::Startswith => {
                todo!("variable predefine function[&startsWith], [&startswith]")
            }
            VariablePredefineFunctions::EndsWith | VariablePredefineFunctions::Endswith => {
                todo!("variable predefine function[&endsWith], [&endswith]")
            }
            VariablePredefineFunctions::Strip | VariablePredefineFunctions::Trim => {
                todo!("variable predefine function[&strip], [$trim]")
            }
            VariablePredefineFunctions::Replace => todo!("variable predefine function[&replace]"),
            VariablePredefineFunctions::ReplaceFirst => {
                todo!("variable predefine function[&replaceFirst]")
            }
            VariablePredefineFunctions::Upper => todo!("variable predefine function[&upper]"),
            VariablePredefineFunctions::Lower => todo!("variable predefine function[&lower]"),
            VariablePredefineFunctions::Contains => todo!("variable predefine function[&contains]"),
            VariablePredefineFunctions::Split => todo!("variable predefine function[&split]"),
            VariablePredefineFunctions::Concat => todo!("variable predefine function[&concat]"),
            VariablePredefineFunctions::ConcatWith => {
                todo!("variable predefine function[&concatWith]")
            }
            VariablePredefineFunctions::Join => value.join_of_str_or_vec(",", not_support),
            VariablePredefineFunctions::Distinct => value.distinct_of_vec(not_support),
            VariablePredefineFunctions::Sum => value.sum(not_support),
            VariablePredefineFunctions::Avg => value.avg(not_support),
            VariablePredefineFunctions::Min => value.min_of_vec(not_support_e),
            VariablePredefineFunctions::MinNum => value.min_decimal_of_vec(not_support_e),
            VariablePredefineFunctions::MinDate => value.min_date_of_vec(not_support_e),
            VariablePredefineFunctions::MinDatetime | VariablePredefineFunctions::MinDt => {
                value.min_datetime_of_vec(not_support_e)
            }
            VariablePredefineFunctions::MinTime => value.min_time(not_support_e),
            VariablePredefineFunctions::Max => value.max(not_support_e),
            VariablePredefineFunctions::MaxNum => value.max_decimal_of_vec(not_support_e),
            VariablePredefineFunctions::MaxDate => value.max_date_of_vec(not_support_e),
            VariablePredefineFunctions::MaxDatetime | VariablePredefineFunctions::MaxDt => {
                value.max_datetime_of_vec(not_support_e)
            }
            VariablePredefineFunctions::MaxTime => value.max_time_of_vec(not_support_e),
            VariablePredefineFunctions::FromCurrentContext => {
                todo!("variable predefine function[&cur]")
            }
            VariablePredefineFunctions::FromPreviousTriggerData => {
                todo!("variable predefine function[&old]")
            }
            VariablePredefineFunctions::DayDiff => todo!("variable predefine function[&dayDiff]"),
            VariablePredefineFunctions::MonthDiff => {
                todo!("variable predefine function[&monthDiff]")
            }
            VariablePredefineFunctions::YearDiff => todo!("variable predefine function[&yearDiff]"),
            VariablePredefineFunctions::MoveDate => todo!("variable predefine function[&moveDate]"),
            VariablePredefineFunctions::DateFormat => {
                todo!("variable predefine function[&fmtDate]")
            }
            VariablePredefineFunctions::Now => todo!("variable predefine function[&now]"),
        }
    }
}
