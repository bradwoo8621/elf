use crate::{ArcTopicDataValue, FuncDataPath, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::VariablePredefineFunctions;
use std::sync::Arc;

pub struct InMemoryFuncCall<'a> {
    path: &'a FuncDataPath,
}

/// property
impl<'a> InMemoryFuncCall<'a> {
    pub fn path(&self) -> &'a FuncDataPath {
        &self.path
    }

    pub fn this_path(&self) -> String {
        self.path.this_path()
    }

    pub fn full_path(&self) -> String {
        self.path.full_path()
    }

    pub fn func(&self) -> &VariablePredefineFunctions {
        self.path.func()
    }
}

/// compute
impl<'a> InMemoryFuncCall<'a> {
    fn do_compute(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match self.path.func() {
            VariablePredefineFunctions::NextSeq => self.context_disallowed(),
            VariablePredefineFunctions::Count => self.resolve_count_of_vec_or_map(context, params),
            VariablePredefineFunctions::Length | VariablePredefineFunctions::Len => {
                self.resolve_length_of_str_or_num(context, params)
            }
            VariablePredefineFunctions::Slice | VariablePredefineFunctions::Substr => {
                self.resolve_slice_of_str(context, params)
            }
            VariablePredefineFunctions::Find | VariablePredefineFunctions::Index => {
                self.resolve_find_of_str(context, params)
            }
            VariablePredefineFunctions::StartsWith | VariablePredefineFunctions::Startswith => {
                self.resolve_starts_with_of_str(context, params)
            }
            VariablePredefineFunctions::EndsWith | VariablePredefineFunctions::Endswith => {
                self.resolve_ends_with_of_str(context, params)
            }
            VariablePredefineFunctions::Strip | VariablePredefineFunctions::Trim => {
                self.resolve_trim_of_str(context, params)
            }
            VariablePredefineFunctions::Replace => self.resolve_replace_of_str(context, params),
            VariablePredefineFunctions::ReplaceFirst => {
                self.resolve_replace_first_of_str(context, params)
            }
            VariablePredefineFunctions::Upper => self.resolve_upper_of_str(context, params),
            VariablePredefineFunctions::Lower => self.resolve_lower_of_str(context, params),
            VariablePredefineFunctions::Contains => self.resolve_contains_of_str(context, params),
            VariablePredefineFunctions::Split => self.resolve_split_of_str(context, params),
            VariablePredefineFunctions::Concat => {
                self.resolve_concat_of_non_vec_or_map(context, params)
            }
            VariablePredefineFunctions::ConcatWith => {
                self.resolve_concat_with_of_non_vec_or_map(context, params)
            }
            VariablePredefineFunctions::Join => self.resolve_join_of_str_or_vec(context, params),
            // VariablePredefineFunctions::Distinct => self.resolve_distinct(context, params),
            // VariablePredefineFunctions::Sum => self.resolve_sum(context, params),
            // VariablePredefineFunctions::Avg => self.resolve_avg(context, params),
            // VariablePredefineFunctions::Max => self.resolve_max(context, params),
            // VariablePredefineFunctions::MaxNum => self.resolve_max_num(context, params),
            // VariablePredefineFunctions::MaxDate => self.resolve_max_date(context, params),
            // VariablePredefineFunctions::MaxDatetime => self.resolve_max_datetime(context, params),
            // VariablePredefineFunctions::MaxDt => self.resolve_max_datetime(context, params),
            // VariablePredefineFunctions::MaxTime => self.resolve_max_time(context, params),
            // VariablePredefineFunctions::Min => self.resolve_min(context, params),
            // VariablePredefineFunctions::MinNum => self.resolve_min_num(context, params),
            // VariablePredefineFunctions::MinDate => self.resolve_min_date(context, params),
            // VariablePredefineFunctions::MinDatetime => self.resolve_min_datetime(context, params),
            // VariablePredefineFunctions::MinDt => self.resolve_min_datetime(context, params),
            // VariablePredefineFunctions::MinTime => self.resolve_min_time(context, params),
            VariablePredefineFunctions::FromCurrentTriggerData => self.context_disallowed(),
            VariablePredefineFunctions::FromPreviousTriggerData => self.context_disallowed(),
            // VariablePredefineFunctions::DayDiff => self.resolve_day_diff(context, params),
            // VariablePredefineFunctions::MonthDiff => self.resolve_month_diff(context, params),
            // VariablePredefineFunctions::YearDiff => self.resolve_year_diff(context, params),
            // VariablePredefineFunctions::MoveDate => self.resolve_move_date(context, params),
            // VariablePredefineFunctions::DateFormat => self.resolve_date_format(context, params),
            VariablePredefineFunctions::Now => self.context_disallowed(),
            _ => self.context_disallowed(),
        }
    }

    /// compute the function value by given context and parameters.
    /// will check the min/max parameter count
    pub fn compute(
        path: &'a FuncDataPath,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let param_count = params.len();
        let func = path.func();
        let min_param_count = func.min_param_count();
        if param_count < min_param_count {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Function[path={}, name={}] has no enough parameters, at least {} are required, but only {} are currently provided.",
                path.full_path(),
                func, min_param_count, param_count
            ));
        }
        if let Some(max_param_count) = func.max_param_count() {
            if param_count > max_param_count {
                return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Function[path={}, name={}] has too many parameters, at most {} are accepted, but {} are currently provided.",
                    path.full_path(),
                    func, max_param_count, param_count
                ));
            }
        }

        Self { path }.do_compute(context, params)
    }
}
