use crate::{BaseDataModel, ModelErrorCode, Storable, SubscriptionEventId, TenantId, UserId};
use chrono::{NaiveDate, NaiveDateTime};
use elf_base::serde::{option_naive_date, option_naive_datetime};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum SubscriptionEventJobLockStatus {
    Ready,
    #[display = "fail"]
    Failed,
    Success,
}

pub type SubscriptionEventLockId = String;

#[adapt_model(storable)]
pub struct SubscriptionEventLock {
    pub subscription_event_lock_id: Option<SubscriptionEventLockId>,
    pub tenant_id: Option<TenantId>,
    pub subscription_event_id: Option<SubscriptionEventId>,
    #[serde(with = "option_naive_date")]
    pub process_date: Option<NaiveDate>,
    pub status: Option<SubscriptionEventJobLockStatus>,
    /// TODO means who subscribe this event? but assuming it is trigger by predefined scheduler, right?
    pub user_id: Option<UserId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
}
