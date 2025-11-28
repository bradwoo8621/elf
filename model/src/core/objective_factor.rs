use crate::{
    BaseDataModel, ComputedObjectiveParameter, IndicatorId, ObjectiveParameterJoint, Storable,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveFactorKind {
    Indicator,
    Computed,
}

pub type ObjectiveFactorId = String;
pub type ObjectiveFactorName = String;

#[adapt_model(storable)]
pub struct ObjectiveFactorOnIndicator {
    pub uuid: Option<ObjectiveFactorId>,
    pub kind: Option<ObjectiveFactorKind>,
    pub name: Option<ObjectiveFactorName>,
    pub formula: Option<ComputedObjectiveParameter>,
    pub indicator_id: Option<IndicatorId>,
    pub conditional: Option<bool>,
    /// objective variables are available in constant value
    pub filter: Option<ObjectiveParameterJoint>,
}

impl ObjectiveFactorOnIndicator {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveFactorKind::Indicator)
    }

    pub fn to_factor(self) -> ObjectiveFactor {
        ObjectiveFactor::Indicator(self)
    }
}

#[adapt_model(storable)]
pub struct ObjectiveFactorOnComputation {
    pub uuid: Option<ObjectiveFactorId>,
    pub kind: Option<ObjectiveFactorKind>,
    pub name: Option<ObjectiveFactorName>,
    pub formula: Option<ComputedObjectiveParameter>,
}

impl ObjectiveFactorOnComputation {
    pub fn init() -> Self {
        Self::new().kind(ObjectiveFactorKind::Computed)
    }

    pub fn to_factor(self) -> ObjectiveFactor {
        ObjectiveFactor::Computed(self)
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ObjectiveFactor {
    Indicator(ObjectiveFactorOnIndicator),
    Computed(ObjectiveFactorOnComputation),
}
