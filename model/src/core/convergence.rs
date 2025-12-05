use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, BucketId, ObjectiveId, ObjectiveTargetId, OptimisticLock, StdErrCode,
    StdErrorCode, StdR, Storable, TenantBasedTuple, TenantId, Tuple, UserGroupId, UserId,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum ConvergenceVariableType {
    Timeframe,
    Bucket,
    FreeWalk,
}

#[derive(Display, Serde, StrEnum)]
pub enum ConvergenceVariableAxis {
    X,
    Y,
}

#[derive(Display, Serde, StrEnum)]
pub enum ConvergenceTimeframeVariableKind {
    Year,
    HalfYear,
    Quarter,
    Month,
    Week,
    Day,
}

#[adapt_model(storable)]
pub struct TimeframeConvergenceVariableValue {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[adapt_model(storable)]
pub struct ConvergenceTimeframeVariable {
    pub uuid: Option<ConvergenceVariableId>,
    pub r#type: Option<ConvergenceVariableType>,
    pub name: Option<String>,
    pub axis: Option<ConvergenceVariableAxis>,
    /// use [kind] and [till] to compute values
    pub kind: Option<ConvergenceTimeframeVariableKind>,
    pub till: Option<String>,
    pub times: Option<i32>,
    pub values: Option<Vec<TimeframeConvergenceVariableValue>>,
}

impl ConvergenceTimeframeVariable {
    pub fn init() -> Self {
        Self::new().r#type(ConvergenceVariableType::Timeframe)
    }

    pub fn to_variable(self) -> ConvergenceVariable {
        ConvergenceVariable::Timeframe(self)
    }
}

#[adapt_model(storable)]
pub struct ConvergenceBucketVariable {
    pub uuid: Option<ConvergenceVariableId>,
    pub r#type: Option<ConvergenceVariableType>,
    pub name: Option<String>,
    pub axis: Option<ConvergenceVariableAxis>,
    pub bucket_id: Option<BucketId>,
}

impl ConvergenceBucketVariable {
    pub fn init() -> Self {
        Self::new().r#type(ConvergenceVariableType::Bucket)
    }

    pub fn to_variable(self) -> ConvergenceVariable {
        ConvergenceVariable::Bucket(self)
    }
}

#[adapt_model(storable)]
pub struct ConvergenceFreeWalkVariable {
    pub uuid: Option<ConvergenceVariableId>,
    pub r#type: Option<ConvergenceVariableType>,
    pub name: Option<String>,
    pub axis: Option<ConvergenceVariableAxis>,
    pub values: Option<Vec<String>>,
}

impl ConvergenceFreeWalkVariable {
    pub fn init() -> Self {
        Self::new().r#type(ConvergenceVariableType::FreeWalk)
    }

    pub fn to_variable(self) -> ConvergenceVariable {
        ConvergenceVariable::FreeWalk(self)
    }
}

pub type ConvergenceVariableId = String;

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "type")]
pub enum ConvergenceVariable {
    #[serde(rename = "timeframe")]
    Timeframe(ConvergenceTimeframeVariable),
    #[serde(rename = "bucket")]
    Bucket(ConvergenceBucketVariable),
    #[serde(rename = "free-walk")]
    FreeWalk(ConvergenceFreeWalkVariable),
}

pub const CONVERGENCE_TARGET_VARIABLE_MAPPING_IGNORED: &'static str = "#";

pub type ConvergenceTargetVariableMappingId = String;

#[adapt_model(storable)]
pub struct ConvergenceTargetVariableMapping {
    pub uuid: Option<ConvergenceTargetVariableMappingId>,
    pub objective_variable_name: Option<String>,
    /// [CONVERGENCE_TARGET_VARIABLE_MAPPING_IGNORED]
    pub variable_id: Option<ConvergenceVariableId>,
}

impl ConvergenceTargetVariableMapping {
    pub fn variable_mapping_ignored(self) -> bool {
        if let Some(variable_id) = self.variable_id {
            variable_id == CONVERGENCE_TARGET_VARIABLE_MAPPING_IGNORED
        } else {
            false
        }
    }
}

pub type ConvergenceTargetId = String;

#[adapt_model(storable)]
pub struct ConvergenceTarget {
    uuid: Option<ConvergenceTargetId>,
    objective_id: Option<ObjectiveId>,
    target_id: Option<ObjectiveTargetId>,
    use_time_frame: Option<bool>,
    mapping: Option<Vec<ConvergenceTargetVariableMapping>>,
    /// starts from 0
    row: Option<u32>,
    /// starts from 0
    col: Option<u32>,
}

pub type ConvergenceId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Convergence {
    convergence_id: Option<ConvergenceId>,
    name: Option<String>,
    description: Option<String>,
    variables: Option<Vec<ConvergenceVariable>>,
    targets: Option<Vec<ConvergenceTarget>>,
    group_ids: Option<Vec<UserGroupId>>,
}
