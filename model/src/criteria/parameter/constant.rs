use crate::serde_for_enum;
use crate::BaseDataModel;
use std::fmt;

pub enum VariablePredefineFunctions {
    // Sequence functions
    NextSeq,
    // Aggregation functions
    Count,
    // String functions
    Length,
    Join,
    // Statistical functions
    Sum,
    Max,
    Min,
    // Retrieve value from previous trigger data
    FromPreviousTriggerData,
    // Date related functions
    DayDiff,
    MonthDiff,
    YearDiff,
    MoveDate,
    DateFormat,
    Now,
}

impl fmt::Display for VariablePredefineFunctions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariablePredefineFunctions::NextSeq => write!(f, "&nextSeq"),
            VariablePredefineFunctions::Count => write!(f, "&count"),
            VariablePredefineFunctions::Length => write!(f, "&length"),
            VariablePredefineFunctions::Join => write!(f, "&join"),
            VariablePredefineFunctions::Sum => write!(f, "&sum"),
            VariablePredefineFunctions::Max => write!(f, "&max"),
            VariablePredefineFunctions::Min => write!(f, "&min"),
            VariablePredefineFunctions::FromPreviousTriggerData => write!(f, "&old"),
            VariablePredefineFunctions::DayDiff => write!(f, "&dayDiff"),
            VariablePredefineFunctions::MonthDiff => write!(f, "&monthDiff"),
            VariablePredefineFunctions::YearDiff => write!(f, "&yearDiff"),
            VariablePredefineFunctions::MoveDate => write!(f, "&moveDate"),
            VariablePredefineFunctions::DateFormat => write!(f, "&fmtDate"),
            VariablePredefineFunctions::Now => write!(f, "&now"),
        }
    }
}

serde_for_enum! {
    VariablePredefineFunctions {
        NextSeq => "&nextSeq",
        Count => "&count",
        Length => "&length",
        Join => "&join",
        Sum => "&sum",
        Max => "&max",
        Min => "&min",
        FromPreviousTriggerData => "&old",
        DayDiff => "&dayDiff",
        MonthDiff => "&monthDiff",
        YearDiff => "&yearDiff",
        MoveDate => "&moveDate",
        DateFormat => "&fmtDate",
        Now => "&now",
    }
}

/// string stands for an expression to retrieve some value
/// might include function calls, see [VariablePredefineFunctions]
pub struct ConstantParameter {
    pub value: Option<String>,
}

impl BaseDataModel for ConstantParameter {}
