use crate::{BaseDataModel, StdErrCode, StdErrorCode, StdR, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveTimeframeKind {
    None,
    Year,
    HalfYear,
    Quarter,
    Month,
    WeekOfYear,
    DayOfMonth,
    DayOfWeek,
    LastNYears,
    LastNMonths,
    LastNWeeks,
    LastNDays,
}

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveTimeframeTill {
    Now,
    LastCompleteCycle,
    Specified,
}

#[adapt_model(storable)]
pub struct ObjectiveTimeframe {
    /// is target in time frame, normally is
    pub kind: Option<ObjectiveTimeframeKind>,
    /// only available if kind is LAST_N-* types, should be a positive value
    pub last_n: Option<String>,
    /// time frame is cut off till when
    pub till: Option<ObjectiveTimeframeTill>,
    /// specify the till time when [till] is [ObjectiveTimeframeTill::Specified]
    pub specified_till: Option<String>,
}
