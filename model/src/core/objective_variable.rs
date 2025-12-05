use crate::{BaseDataModel, BucketId, StdErrCode, StdErrorCode, StdR, Storable};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveVariableKind {
    #[display = "value"]
    SingleValue,
    Bucket,
    Range,
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnValue {
    pub name: Option<String>,
    pub kind: Option<ObjectiveVariableKind>,
    pub value: Option<String>,
}

impl ObjectiveVariableOnValue {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveVariableKind::SingleValue)
    }

    pub fn to_variable(self) -> ObjectiveVariable {
        ObjectiveVariable::SingleValue(self)
    }
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnBucket {
    pub name: Option<String>,
    pub kind: Option<ObjectiveVariableKind>,
    pub bucket_id: Option<BucketId>,
    pub segment_name: Option<String>,
}

impl ObjectiveVariableOnBucket {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveVariableKind::Bucket)
    }

    pub fn to_variable(self) -> ObjectiveVariable {
        ObjectiveVariable::Bucket(self)
    }
}

#[adapt_model(storable)]
pub struct ObjectiveVariableOnRange {
    pub name: Option<String>,
    pub kind: Option<ObjectiveVariableKind>,
    pub min: Option<String>,
    pub include_min: Option<bool>,
    pub max: Option<String>,
    pub include_max: Option<bool>,
}

impl ObjectiveVariableOnRange {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveVariableKind::Range)
    }

    pub fn to_variable(self) -> ObjectiveVariable {
        ObjectiveVariable::Range(self)
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ObjectiveVariable {
    SingleValue(ObjectiveVariableOnValue),
    Bucket(ObjectiveVariableOnBucket),
    Range(ObjectiveVariableOnRange),
}
