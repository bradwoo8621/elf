use crate::{BaseDataModel, Parameter, ParameterKind, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
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
