use crate::serde::{option_naive_date, option_naive_datetime};
use crate::{
    Auditable, BaseDataModel, OptimisticLock, ParameterJoint, PipelineId, Storable,
    TenantBasedTuple, TenantId, TopicId, Tuple, UserId,
};
use chrono::{NaiveDate, NaiveDateTime};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum TopicSnapshotFrequency {
    Daily,
    Weekly,
    Monthly,
}

pub type TopicSnapshotSchedulerId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct TopicSnapshotScheduler {
    pub scheduler_id: Option<TopicSnapshotSchedulerId>,
    pub topic_id: Option<TopicId>,
    pub target_topic_name: Option<String>,
    pub target_topic_id: Option<TopicId>,
    pub pipeline_id: Option<PipelineId>,
    pub frequency: Option<TopicSnapshotFrequency>,
    pub filter: Option<ParameterJoint>,
    /// only for weekly
    pub weekday: Option<i8>,
    /// only for monthly
    pub day: Option<i8>,
    pub hour: Option<i8>,
    pub minute: Option<i8>,
    pub enabled: Option<bool>,
}

#[derive(Display, Serde)]
pub enum TopicSnapshotJobLockStatus {
    Ready,
    #[display = "fail"]
    Failed,
    Success,
}

pub type TopicSnapshotJobLockId = String;

#[adapt_model(storable)]
pub struct TopicSnapshotJobLock {
    pub lock_id: Option<TopicSnapshotJobLockId>,
    pub scheduler_id: Option<TopicSnapshotSchedulerId>,
    // TODO means snapshot of data in appointed topic, and belongs to this given tenant?
    pub tenant_id: Option<TenantId>,
    pub frequency: Option<TopicSnapshotFrequency>,
    #[serde(with = "option_naive_date")]
    pub process_date: Option<NaiveDate>,
    pub row_count: Option<u32>,
    pub status: Option<TopicSnapshotJobLockStatus>,
    /// TODO means who take this snapshot? but assuming it is trigger by predefined scheduler, right?
    pub user_id: Option<UserId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
}
