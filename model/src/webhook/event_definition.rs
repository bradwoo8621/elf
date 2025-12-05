use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, StdErrCode, StdErrorCode, StdR, Storable, Tuple,
    UserId, UserRole,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum EventType {
    System,
    Business,
}

#[derive(Display, Serde, StrEnum)]
pub enum EventSource {
    Subject,
    #[display = "objective_analysis"]
    ObjectiveAnalysis,
}

pub type EventDefinitionId = String;

#[adapt_model(opt_lock, tuple)]
pub struct EventDefinition {
    pub event_definition_id: Option<EventDefinitionId>,
    pub event_code: Option<String>,
    pub event_name: Option<String>,
    pub event_type: Option<EventType>,
    pub event_source: Option<EventSource>,
    pub role: Option<UserRole>,
}
