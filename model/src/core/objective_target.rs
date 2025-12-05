use crate::{
    BaseDataModel, ComputedObjectiveParameter, ObjectiveFactorId, StdErrCode, StdErrorCode, StdR,
    Storable,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum ObjectiveTargetBetterSide {
    Less,
    More,
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ObjectiveTargetAsIs {
    Parameter(ComputedObjectiveParameter),
    Factor(ObjectiveFactorId),
}

pub type ObjectiveTargetId = String;

#[adapt_model(storable)]
pub struct ObjectiveTarget {
    pub uuid: Option<ObjectiveTargetId>,
    pub name: Option<String>,
    /// to be value, should be a numeric value, a percentage value
    pub tobe: Option<String>,
    /// as is formula
    pub asis: Option<ObjectiveTargetAsIs>,
    /// which side is better, with computed as is value vs to be value.
    pub better_side: Option<ObjectiveTargetBetterSide>,
    /// this July vs this June if time frame is on month, month-on-month
    pub ask_previous_cycle: Option<bool>,
    /// this July vs last July if time frame is on month, year-on-year
    pub ask_chain_cycle: Option<bool>,
}
