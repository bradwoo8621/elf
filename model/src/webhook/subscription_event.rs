use crate::{
	Auditable, BaseDataModel, EventSource, ModelErrorCode, NotificationDefinitionId,
	OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum SubscriptionEventFrequency {
    Daily,
    Weekly,
    Monthly,
}

pub type SubscriptionEventId = String;
pub type SubscriptionEventContentType = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct SubscriptionEvent {
    pub subscription_event_id: Option<SubscriptionEventId>,
    /// eventId: EventDefinitionId = None
    pub event_code: Option<String>,
    pub event_source: Option<EventSource>,
    pub notification_id: Option<NotificationDefinitionId>,
    pub source_id: Option<String>,
    pub user_id: Option<UserId>,
    pub content_type: Option<SubscriptionEventContentType>,
    /// only for weekly
    pub weekday: Option<String>,
    /// only for monthly
    pub day: Option<String>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub enabled: Option<bool>,
    pub status: Option<bool>,
    pub frequency: Option<SubscriptionEventFrequency>,
}
