use crate::{
    BaseDataModel, BucketId, FactorOrObjectiveFactorIdOrSubjectDatasetColumnId,
    ObjectiveParameterJoint, StdErrCode, StdErrorCode, StdR, Storable,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveParameterType {
    Refer,
    Constant,
    Computed,
    /// only available on factor indicator filter
    Bucket,
    /// only available on factor indicator filter
    #[display = "time-frame"]
    Timeframe,
}

/// it's a multiple purposes object.
///	when it is used in factor/target formula, {@link #uuid} should refer to another objective factor.
///	and when it is used in factor filter, {@link #uuid} should refer to factor from topic or column from subject dataset.
#[adapt_model(storable)]
pub struct ReferObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub uuid: Option<FactorOrObjectiveFactorIdOrSubjectDatasetColumnId>,
}

impl ReferObjectiveParameter {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveParameterType::Refer)
    }

    pub fn of(uuid: FactorOrObjectiveFactorIdOrSubjectDatasetColumnId) -> Self {
        Self::init().uuid(uuid)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Refer(self)
    }
}

#[adapt_model(storable)]
pub struct ConstantObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub value: Option<String>,
}

impl ConstantObjectiveParameter {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveParameterType::Constant)
    }

    pub fn of(value: String) -> Self {
        Self::init().value(value)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Constant(self)
    }
}

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveFormulaOperator {
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
    Round,
    Floor,
    Ceil,
    Abs,
    Max,
    Min,
    Interpolate,

    CaseThen,
}

#[adapt_model(storable)]
pub struct ObjectiveNoneParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveNoneParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::None)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::None(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveAddParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveAddParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Add)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Add(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveSubtractParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveSubtractParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Subtract)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Subtract(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMultiplyParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveMultiplyParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Multiply)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Multiply(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveDivideParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveDivideParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Divide)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Divide(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveModulusParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveModulusParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Modulus)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Modulus(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveYearOfParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveYearOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::YearOf)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::YearOf(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveHalfYearOfParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveHalfYearOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::HalfYearOf)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::HalfYearOf(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveQuarterOfParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveQuarterOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::QuarterOf)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::QuarterOf(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMonthOfParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveMonthOfParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::MonthOf)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::MonthOf(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveWeekOfYearParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveWeekOfYearParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::WeekOfYear)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::WeekOfYear(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveWeekOfMonthParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveWeekOfMonthParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::WeekOfMonth)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::WeekOfMonth(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveDayOfMonthParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveDayOfMonthParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::DayOfMonth)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::DayOfMonth(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveDayOfWeekParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveDayOfWeekParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::DayOfWeek)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::DayOfWeek(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveRoundParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveRoundParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Round)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Round(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveFloorParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveFloorParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Floor)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Floor(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveCeilParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveCeilParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Ceil)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Ceil(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveAbsParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    /// use [Box<Parameter>] to avoid recursive type size issue.
    /// TODO serde from an array which name is "parameters"
    pub parameter: Option<Box<ObjectiveParameter>>,
}

impl ObjectiveAbsParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Abs)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Abs(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMaxParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveMaxParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Max)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Max(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveMinParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveMinParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Min)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Min(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveInterpolateParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveParameter>>,
}

impl ObjectiveInterpolateParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::Interpolate)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::Interpolate(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[adapt_model(storable)]
pub struct ObjectiveCaseThenParameterRoute {
    pub conditional: Option<bool>,
    pub on: Option<ObjectiveParameterJoint>,
    #[serde(flatten)]
    pub parameter: Option<ObjectiveParameter>,
}

impl ObjectiveCaseThenParameterRoute {
    /// it is to create the default route (without condition), not the [Default::default]
    pub fn default() -> Self {
        Self::new().conditional(false)
    }

    /// it is to create the conditional route
    pub fn case(joint: ObjectiveParameterJoint) -> Self {
        Self::new().conditional(true).on(joint)
    }
}

#[adapt_model(storable)]
pub struct ObjectiveCaseThenParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub operator: Option<ObjectiveFormulaOperator>,
    pub parameters: Option<Vec<ObjectiveCaseThenParameterRoute>>,
}

impl ObjectiveCaseThenParameter {
    pub fn init() -> Self {
        Self::new()
            .kind(ObjectiveParameterType::Computed)
            .operator(ObjectiveFormulaOperator::CaseThen)
    }

    pub fn to_computed(self) -> ComputedObjectiveParameter {
        ComputedObjectiveParameter::CaseThen(self)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self.to_computed())
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "operator")]
pub enum ComputedObjectiveParameter {
    None(ObjectiveNoneParameter),
    Add(ObjectiveAddParameter),
    Subtract(ObjectiveSubtractParameter),
    Multiply(ObjectiveMultiplyParameter),
    Divide(ObjectiveDivideParameter),
    Modulus(ObjectiveModulusParameter),
    YearOf(ObjectiveYearOfParameter),
    HalfYearOf(ObjectiveHalfYearOfParameter),
    QuarterOf(ObjectiveQuarterOfParameter),
    MonthOf(ObjectiveMonthOfParameter),
    WeekOfYear(ObjectiveWeekOfYearParameter),
    WeekOfMonth(ObjectiveWeekOfMonthParameter),
    DayOfMonth(ObjectiveDayOfMonthParameter),
    DayOfWeek(ObjectiveDayOfWeekParameter),
    Round(ObjectiveRoundParameter),
    Floor(ObjectiveFloorParameter),
    Ceil(ObjectiveCeilParameter),
    Abs(ObjectiveAbsParameter),
    Max(ObjectiveMaxParameter),
    Min(ObjectiveMinParameter),
    Interpolate(ObjectiveInterpolateParameter),
    CaseThen(ObjectiveCaseThenParameter),
}

impl ComputedObjectiveParameter {
    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Computed(self)
    }
}

#[adapt_model(storable)]
pub struct BucketObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
    pub bucket_id: Option<BucketId>,
    pub segment_name: Option<String>,
}

impl BucketObjectiveParameter {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveParameterType::Bucket)
    }

    pub fn of(bucket_id: BucketId) -> Self {
        Self::init().bucket_id(bucket_id)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Bucket(self)
    }
}

/// TODO no other field rather than [kind], weird
#[adapt_model(storable)]
pub struct TimeframeObjectiveParameter {
    pub kind: Option<ObjectiveParameterType>,
}

impl TimeframeObjectiveParameter {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveParameterType::Timeframe)
    }

    pub fn to_parameter(self) -> ObjectiveParameter {
        ObjectiveParameter::Timeframe(self)
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ObjectiveParameter {
    Refer(ReferObjectiveParameter),
    Constant(ConstantObjectiveParameter),
    Computed(ComputedObjectiveParameter),
    Bucket(BucketObjectiveParameter),
    Timeframe(TimeframeObjectiveParameter),
}
