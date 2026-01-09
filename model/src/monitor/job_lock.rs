use crate::{
	BaseDataModel, ModelErrorCode, MonitorRuleStatisticalInterval, Storable, TenantId, TopicId,
	UserId,
};
use chrono::{NaiveDate, NaiveDateTime};
use elf_base::serde::{option_naive_date, option_naive_datetime};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum MonitorJobLockStatus {
    Ready,
    #[display = "fail"]
    Failed,
    Success,
}

pub type MonitorJobLockId = String;

#[adapt_model(storable)]
pub struct MonitorJobLock {
    pub lock_id: Option<MonitorJobLockId>,
    pub tenant_id: Option<TenantId>,
    pub topic_id: Option<TopicId>,
    pub frequency: Option<MonitorRuleStatisticalInterval>,
    #[serde(with = "option_naive_date")]
    pub process_date: Option<NaiveDate>,
    pub status: Option<MonitorJobLockStatus>,
    pub user_id: Option<UserId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
}
