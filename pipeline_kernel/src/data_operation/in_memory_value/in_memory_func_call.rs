use crate::{ArcTopicDataValue, FuncDataPath, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::VariablePredefineFunctions;
use std::sync::Arc;

pub struct InMemoryFuncCall<'a> {
    path: &'a FuncDataPath,
}

impl<'a> InMemoryFuncCall<'a> {
    fn context_disallowed<R>(&self) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Function[path={}, name={}] doesn't allow context.",
            self.path.full_path(),
            self.path.this_path()
        ))
    }

    fn do_compute(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<ArcTopicDataValue> {
        match self.path.func() {
            VariablePredefineFunctions::NextSeq => self.context_disallowed(),
            VariablePredefineFunctions::Count => self.resolve_count(context, params),
            VariablePredefineFunctions::Length |
            VariablePredefineFunctions::Len => self.resolve_length(context, params),
            VariablePredefineFunctions::Slice |
            VariablePredefineFunctions::Substr => self.resolve_slice(context, params),
            VariablePredefineFunctions::Find |
            VariablePredefineFunctions::Index => self.resolve_find(context, params),
            VariablePredefineFunctions::StartsWith |
            VariablePredefineFunctions::Startswith => self.resolve_starts_with(context, params),
            VariablePredefineFunctions::EndsWith |
            VariablePredefineFunctions::Endswith => self.resolve_ends_with(context, params),
            VariablePredefineFunctions::Strip |
            VariablePredefineFunctions::Trim => self.resolve_trim(context, params),
            VariablePredefineFunctions::Replace => self.resolve_replace(context, params),
            VariablePredefineFunctions::ReplaceFirst => self.resolve_replace_first(context, params),
            VariablePredefineFunctions::Upper => self.resolve_upper(context, params),
            VariablePredefineFunctions::Lower => self.resolve_lower(context, params),
            VariablePredefineFunctions::Contains => self.resolve_contains(context, params),
            VariablePredefineFunctions::Split => self.resolve_split(context, params),
            VariablePredefineFunctions::Concat => self.resolve_concat(context, params),
            VariablePredefineFunctions::ConcatWith => self.resolve_concat_with(context, params),
            VariablePredefineFunctions::Join => self.resolve_join(context, params),
            VariablePredefineFunctions::Distinct => self.resolve_distinct(context, params),
            VariablePredefineFunctions::Sum => {}
            VariablePredefineFunctions::Avg => {}
            VariablePredefineFunctions::Max => {}
            VariablePredefineFunctions::MaxNum => {}
            VariablePredefineFunctions::MaxDate => {}
            VariablePredefineFunctions::MaxDatetime => {}
            VariablePredefineFunctions::MaxDt => {}
            VariablePredefineFunctions::MaxTime => {}
            VariablePredefineFunctions::Min => {}
            VariablePredefineFunctions::MinNum => {}
            VariablePredefineFunctions::MinDate => {}
            VariablePredefineFunctions::MinDatetime => {}
            VariablePredefineFunctions::MinDt => {}
            VariablePredefineFunctions::MinTime => {}
            VariablePredefineFunctions::FromCurrentTriggerData => self.context_disallowed(),
            VariablePredefineFunctions::FromPreviousTriggerData => self.context_disallowed(),
            VariablePredefineFunctions::DayDiff => {}
            VariablePredefineFunctions::MonthDiff => {}
            VariablePredefineFunctions::YearDiff => {}
            VariablePredefineFunctions::MoveDate => {}
            VariablePredefineFunctions::DateFormat => {}
            VariablePredefineFunctions::Now => self.context_disallowed(),
        }
    }

    pub fn compute(
        func: &'a FuncDataPath,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<ArcTopicDataValue> {
        Self { path: func }.do_compute(context, params)
    }
}
