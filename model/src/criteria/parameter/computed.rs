use crate::{serde_for_enum, BaseDataModel, Parameter, ParameterJoint};
use watchmen_model_marco::{adapt_model, Display};

#[derive(Display)]
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

serde_for_enum! {
    ParameterComputeType {
        None => "none",
        Add => "add",
        Subtract => "subtract",
        Multiply => "multiply",
        Divide => "divide",
        Modulus => "modulus",
        YearOf => "year-of",
        HalfYearOf => "half-year-of",
        QuarterOf => "quarter-of",
        MonthOf => "month-of",
        WeekOfYear => "week-of-year",
        WeekOfMonth => "week-of-month",
        DayOfMonth => "day-of-month",
        DayOfWeek => "day-of-week",
        CaseThen => "case-then",
    }
}

/// use [Box<Parameter>] to avoid recursive type size issue
#[adapt_model(bdm)]
pub enum ComputedParameter {
    // math operations
    Add(Option<Vec<Parameter>>),
    Subtract(Option<Vec<Parameter>>),
    Multiply(Option<Vec<Parameter>>),
    Divide(Option<Vec<Parameter>>),
    Modulus(Option<Vec<Parameter>>),
    // date related operations
    YearOf(Option<Box<Parameter>>),
    HalfYearOf(Option<Box<Parameter>>),
    QuarterOf(Option<Box<Parameter>>),
    MonthOf(Option<Box<Parameter>>),
    WeekOfYear(Option<Box<Parameter>>),
    WeekOfMonth(Option<Box<Parameter>>),
    DayOfMonth(Option<Box<Parameter>>),
    DayOfWeek(Option<Box<Parameter>>),
    // conditional operation
    CaseThen(Option<Vec<(Parameter, Option<ParameterJoint>)>>),
}

impl ComputedParameter {
    pub fn compute_type(&self) -> ParameterComputeType {
        match self {
            ComputedParameter::Add(_) => ParameterComputeType::Add,
            ComputedParameter::Subtract(_) => ParameterComputeType::Subtract,
            ComputedParameter::Multiply(_) => ParameterComputeType::Multiply,
            ComputedParameter::Divide(_) => ParameterComputeType::Divide,
            ComputedParameter::Modulus(_) => ParameterComputeType::Modulus,
            ComputedParameter::YearOf(_) => ParameterComputeType::YearOf,
            ComputedParameter::HalfYearOf(_) => ParameterComputeType::HalfYearOf,
            ComputedParameter::QuarterOf(_) => ParameterComputeType::QuarterOf,
            ComputedParameter::MonthOf(_) => ParameterComputeType::MonthOf,
            ComputedParameter::WeekOfYear(_) => ParameterComputeType::WeekOfYear,
            ComputedParameter::WeekOfMonth(_) => ParameterComputeType::WeekOfMonth,
            ComputedParameter::DayOfMonth(_) => ParameterComputeType::DayOfMonth,
            ComputedParameter::DayOfWeek(_) => ParameterComputeType::DayOfWeek,
            ComputedParameter::CaseThen(_) => ParameterComputeType::CaseThen,
        }
    }
}
