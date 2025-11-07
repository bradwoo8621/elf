use crate::{serde_for_enum, BaseDataModel};
use watchmen_model_marco::{adapt_model, DisplayWithAmpersandPrefix};

#[derive(DisplayWithAmpersandPrefix)]
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
#[adapt_model(bdm)]
pub struct ConstantParameter {
    pub value: Option<String>,
}
