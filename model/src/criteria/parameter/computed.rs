use crate::{BaseDataModel, ModelErrorCode, Parameter, ParameterJoint, ParameterKind, Storable};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};
use serde::{Deserialize, Serialize};

#[derive(Display, Serde, Debug, StrEnum)]
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
pub struct NoneParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl NoneParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::None)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::None(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct AddParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl AddParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::Add)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::Add(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct SubtractParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl SubtractParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::Subtract)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::Subtract(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct MultiplyParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl MultiplyParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::Multiply)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::Multiply(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct DivideParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl DivideParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::Divide)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::Divide(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ModulusParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<Parameter>>,
}

impl ModulusParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::Modulus)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::Modulus(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct YearOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl YearOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::YearOf)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::YearOf(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct HalfYearOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl HalfYearOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::HalfYearOf)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::HalfYearOf(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct QuarterOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl QuarterOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::QuarterOf)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::QuarterOf(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct MonthOfParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl MonthOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::MonthOf)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::MonthOf(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct WeekOfYearParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl WeekOfYearParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::WeekOfYear)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::WeekOfYear(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct WeekOfMonthParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl WeekOfMonthParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::WeekOfMonth)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::WeekOfMonth(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct DayOfMonthParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl DayOfMonthParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::DayOfMonth)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::DayOfMonth(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct DayOfWeekParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<Parameter>>,
}

impl DayOfWeekParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::DayOfWeek)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::DayOfWeek(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct CaseThenParameterRoute {
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
    #[serde(flatten)]
    pub parameter: Option<Parameter>,
}

impl CaseThenParameterRoute {
    /// it is to create the default route (without condition), not the [Default::default]
    pub fn default() -> Self {
        Self::new().conditional(false)
    }

    /// it is to create the conditional route
    pub fn case(joint: ParameterJoint) -> Self {
        Self::new().conditional(true).on(joint)
    }
}

#[adapt_model(storable)]
pub struct CaseThenParameter {
    pub kind: Option<ParameterKind>,
    pub r#type: Option<ParameterComputeType>,
    pub parameters: Option<Vec<CaseThenParameterRoute>>,
}

impl CaseThenParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ParameterKind::Computed)
            .r#type(ParameterComputeType::CaseThen)
    }

    pub fn to_computed(self) -> ComputedParameter {
        ComputedParameter::CaseThen(self)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self.to_computed())
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "type")]
pub enum ComputedParameter {
    #[serde(rename = "none")]
    None(NoneParameter),
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

impl ComputedParameter {
    pub fn to_parameter(self) -> Parameter {
        Parameter::Computed(self)
    }
}
