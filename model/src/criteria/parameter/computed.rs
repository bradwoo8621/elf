use crate::{BaseDataModel, Parameter, ParameterJoint, ParameterKind, Storable};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ParameterComputeType {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    YearOf,
    HalfYearOf,
    QuarterOf,
    MonthOf,
    WeekOfYear,
    WeekOfMonth,
    DayOfMonth,
    DayOfWeek,
    CaseThen,
}

#[adapt_model(storable)]
pub struct AddParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

#[adapt_model(storable)]
pub struct SubtractParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

#[adapt_model(storable)]
pub struct MultiplyParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

#[adapt_model(storable)]
pub struct DivideParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

#[adapt_model(storable)]
pub struct ModulusParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

#[adapt_model(storable)]
pub struct YearOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct HalfYearOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct QuarterOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct MonthOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct WeekOfYearParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct WeekOfMonthParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct DayOfMonthParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct DayOfWeekParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

#[adapt_model(storable)]
pub struct CaseThenParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<(Parameter, Option<ParameterJoint>)>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComputedParameter {
    // math operations
    #[serde(rename = "add")]
    Add(AddParameter),
    #[serde(rename = "subtract")]
    Subtract(SubtractParameter),
    #[serde(rename = "multiply")]
    Multiply(MultiplyParameter),
    #[serde(rename = "divide")]
    Divide(DivideParameter),
    #[serde(rename = "modulus")]
    Modulus(ModulusParameter),
    // date related operations
    #[serde(rename = "year-of")]
    YearOf(YearOfParameter),
    #[serde(rename = "half-year-of")]
    HalfYearOf(HalfYearOfParameter),
    #[serde(rename = "quarter-of")]
    QuarterOf(QuarterOfParameter),
    #[serde(rename = "month-of")]
    MonthOf(MonthOfParameter),
    #[serde(rename = "week-of-year")]
    WeekOfYear(WeekOfYearParameter),
    #[serde(rename = "week-of-month")]
    WeekOfMonth(WeekOfMonthParameter),
    #[serde(rename = "day-of-month")]
    DayOfMonth(DayOfMonthParameter),
    #[serde(rename = "day-of-week")]
    DayOfWeek(DayOfWeekParameter),
    // conditional operation
    #[serde(rename = "case-then")]
    CaseThen(CaseThenParameter),
}
