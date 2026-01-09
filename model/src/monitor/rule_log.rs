use crate::{BaseDataModel, FactorId, MonitorRuleCode, Storable, TopicId};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct MonitorRuleLog {
    pub rule_code: Option<MonitorRuleCode>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
    pub count: Option<i32>,
    #[serde(with = "option_naive_datetime")]
    pub last_occurred_time: Option<NaiveDateTime>,
}

#[adapt_model(storable)]
pub struct MonitorRuleLogCriteria {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub rule_code: Option<MonitorRuleCode>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}
