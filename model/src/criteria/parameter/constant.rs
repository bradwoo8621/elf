use crate::BaseDataModel;
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
#[pattern = "ampersand-prefix"]
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

/// string stands for an expression to retrieve some value
/// might include function calls, see [VariablePredefineFunctions]
#[adapt_model(bdm)]
pub struct ConstantParameter {
    pub value: Option<String>,
}
