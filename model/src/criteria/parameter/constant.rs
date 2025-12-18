use crate::{BaseDataModel, Parameter, ParameterKind, StdErrCode, StdErrorCode, StdR, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
#[pattern = "ampersand-prefix"]
pub enum VariablePredefineFunctions {
    // Sequence functions
    /// get next sequence number, [only in-memory]
    NextSeq,
    // Aggregation functions
    /// count of vec or map, [only in-memory]
    Count,
    // String functions
    /// chars count of string or decimal (to string)
    Length,
    /// alias of [Length]
    Len,
    Slice,
    /// alias of [Slice]
    Substr,
    Find,
    /// alias of [Find]
    Index,
    StartsWith,
    /// alias of [StartsWith]
    #[display = "&startswith"]
    Startswith,
    EndsWith,
    /// alias of [EndsWith]
    #[display = "&endswith"]
    Endswith,
    Strip,
    /// alias of [Strip]
    Trim,
    Replace,
    ReplaceFirst,
    Upper,
    Lower,
    Contains,
    Split,
    // concat anything to string
    Concat,
    /// join the elements of vec to a string, [only in-memory]
    Join,
    // Statistical functions
    /// get a distinct vec, [only in-memory]
    Distinct,
    /// sum of elements of vec, [only in-memory]
    Sum,
    /// avg of elements of vec, [only in-memory]
    Avg,
    /// max of elements of vec, [only in-memory]
    Max,
    /// max decimal elements of vec, [only in-memory]
    MaxNum,
    /// max date of elements of vec, [only in-memory]
    MaxDate,
    /// max date time of elements of vec, [only in-memory]
    MaxDatetime,
    /// alias of [MaxDatetime]
    MaxDt,
    /// max time of elements of vec, [only in-memory]
    MaxTime,
    /// min of elements of vec, [only in-memory]
    Min,
    /// min decimal elements of vec, [only in-memory]
    MinNum,
    /// min date of elements of vec, [only in-memory]
    MinDate,
    /// min date time of elements of vec, [only in-memory]
    MinDatetime,
    /// alias of [MinDatetime]
    MinDt,
    /// min time of elements of vec, [only in-memory]
    MinTime,
    // Retrieve value from current context, include variables and current trigger data
    #[display = "&cur"]
    FromCurrentContext,
    // Retrieve value from previous trigger data
    #[display = "&old"]
    FromPreviousTriggerData,
    // Date related functions
    DayDiff,
    MonthDiff,
    YearDiff,
    MoveDate,
    #[display = "&fmtDate"]
    DateFormat,
    Now,
}

impl VariablePredefineFunctions {
    /// function that context disallowed.
    /// e.g. in [a.&length], [a] is the context.
    /// current the following functions cannot have a context
    /// - [&nextSeq],
    /// - [&cur],
    /// - [&old],
    /// - [&now]
    pub fn context_disallowed(&self) -> bool {
        match self {
            VariablePredefineFunctions::NextSeq
            | VariablePredefineFunctions::FromCurrentContext
            | VariablePredefineFunctions::FromPreviousTriggerData
            | VariablePredefineFunctions::Now => true,
            _ => false,
        }
    }

    /// get the minimum parameters count of function,
    /// since the context can be the first parameter of function (unless function disallows context),
    /// the returned count doesn't count in the context parameter.
    /// e.g. [a.&length] is same as [&length(a)], the min params count is 0
    pub fn min_param_count(&self) -> usize {
        match self {
            VariablePredefineFunctions::NextSeq => 0,
            VariablePredefineFunctions::Count => 0,
            VariablePredefineFunctions::Length => 0,
            VariablePredefineFunctions::Len => 0,
            // a.&slice(1), get sub str for a from index 1 (included)
            VariablePredefineFunctions::Slice => 1,
            VariablePredefineFunctions::Substr => 1,
            VariablePredefineFunctions::Find => 1,
            VariablePredefineFunctions::Index => 1,
            VariablePredefineFunctions::StartsWith => 1,
            VariablePredefineFunctions::Startswith => 1,
            VariablePredefineFunctions::EndsWith => 1,
            VariablePredefineFunctions::Endswith => 1,
            VariablePredefineFunctions::Strip => 0,
            VariablePredefineFunctions::Trim => 0,
            VariablePredefineFunctions::Replace => 1,
            VariablePredefineFunctions::ReplaceFirst => 1,
            VariablePredefineFunctions::Upper => 0,
            VariablePredefineFunctions::Lower => 0,
            VariablePredefineFunctions::Contains => 1,
            VariablePredefineFunctions::Split => 0,
            // a.&concat(b): "{a}{b}"
            // &concat(a): "{a}"
            VariablePredefineFunctions::Concat => 1,
            VariablePredefineFunctions::Join => 0,
            VariablePredefineFunctions::Distinct => 0,
            VariablePredefineFunctions::Sum => 0,
            VariablePredefineFunctions::Avg => 0,
            VariablePredefineFunctions::Max => 0,
            VariablePredefineFunctions::MaxNum => 0,
            VariablePredefineFunctions::MaxDate => 0,
            VariablePredefineFunctions::MaxDatetime => 0,
            VariablePredefineFunctions::MaxDt => 0,
            VariablePredefineFunctions::MaxTime => 0,
            VariablePredefineFunctions::Min => 0,
            VariablePredefineFunctions::MinNum => 0,
            VariablePredefineFunctions::MinDate => 0,
            VariablePredefineFunctions::MinDatetime => 0,
            VariablePredefineFunctions::MinDt => 0,
            VariablePredefineFunctions::MinTime => 0,
            VariablePredefineFunctions::FromCurrentContext => 0,
            VariablePredefineFunctions::FromPreviousTriggerData => 0,
            VariablePredefineFunctions::DayDiff => 1,
            VariablePredefineFunctions::MonthDiff => 1,
            VariablePredefineFunctions::YearDiff => 1,
            VariablePredefineFunctions::MoveDate => 1,
            VariablePredefineFunctions::DateFormat => 1,
            VariablePredefineFunctions::Now => 0,
        }
    }

    /// get the maximum parameters count of function, -1 means no limit.
    /// since the context can be the first parameter of function (unless function disallows context),
    /// the returned count doesn't count in the context parameter.
    /// e.g. [a.&length] is same as [&length(a)], the max params count is 0,
    /// e.g. [a.&minNum(b, c, ...)], [&minNum(a, b, c, ...)], any parameters are accepted.
    pub fn max_param_count(&self) -> i64 {
        match self {
            VariablePredefineFunctions::NextSeq => 0,
            VariablePredefineFunctions::Count => 0,
            VariablePredefineFunctions::Length => 0,
            VariablePredefineFunctions::Len => 0,
            // a.&slice(1, 3), get sub str for a from index 1 (included) to index 3 (excluded)
            VariablePredefineFunctions::Slice => 2,
            VariablePredefineFunctions::Substr => 2,
            VariablePredefineFunctions::Find => 1,
            VariablePredefineFunctions::Index => 1,
            VariablePredefineFunctions::StartsWith => 1,
            VariablePredefineFunctions::Startswith => 1,
            VariablePredefineFunctions::EndsWith => 1,
            VariablePredefineFunctions::Endswith => 1,
            VariablePredefineFunctions::Strip => 0,
            VariablePredefineFunctions::Trim => 0,
            VariablePredefineFunctions::Replace => 1,
            VariablePredefineFunctions::ReplaceFirst => 1,
            VariablePredefineFunctions::Upper => 0,
            VariablePredefineFunctions::Lower => 0,
            VariablePredefineFunctions::Contains => 1,
            VariablePredefineFunctions::Split => 0,
            // a.&concat(b): "{a}{b}"
            // &concat(a): "{a}"
            VariablePredefineFunctions::Concat => -1,
            VariablePredefineFunctions::Join => 1,
            VariablePredefineFunctions::Distinct => -1,
            VariablePredefineFunctions::Sum => -1,
            VariablePredefineFunctions::Avg => -1,
            VariablePredefineFunctions::Max => -1,
            VariablePredefineFunctions::MaxNum => -1,
            VariablePredefineFunctions::MaxDate => -1,
            VariablePredefineFunctions::MaxDatetime => -1,
            VariablePredefineFunctions::MaxDt => -1,
            VariablePredefineFunctions::MaxTime => -1,
            VariablePredefineFunctions::Min => -1,
            VariablePredefineFunctions::MinNum => -1,
            VariablePredefineFunctions::MinDate => -1,
            VariablePredefineFunctions::MinDatetime => -1,
            VariablePredefineFunctions::MinDt => -1,
            VariablePredefineFunctions::MinTime => -1,
            VariablePredefineFunctions::FromCurrentContext => 0,
            VariablePredefineFunctions::FromPreviousTriggerData => 0,
            VariablePredefineFunctions::DayDiff => 1,
            VariablePredefineFunctions::MonthDiff => 1,
            VariablePredefineFunctions::YearDiff => 1,
            VariablePredefineFunctions::MoveDate => 1,
            VariablePredefineFunctions::DateFormat => 1,
            VariablePredefineFunctions::Now => 0,
        }
    }
}

/// string stands for an expression to retrieve some value
/// might include function calls, see [VariablePredefineFunctions]
#[adapt_model(storable)]
pub struct ConstantParameter {
    pub kind: Option<ParameterKind>,
    pub value: Option<String>,
}

impl ConstantParameter {
    pub fn init() -> Self {
        Self::new().kind(ParameterKind::Constant)
    }

    pub fn of(value: String) -> Self {
        Self::init().value(value)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Constant(self)
    }
}
