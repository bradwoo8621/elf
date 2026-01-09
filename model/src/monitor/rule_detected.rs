use crate::{BaseDataModel, FactorId, MonitorRuleCode, MonitorRuleSeverity, Storable, TopicId};
use chrono::NaiveDate;
use elf_base::serde::option_naive_date;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct MonitorRuleDetected {
    pub rule_code: Option<MonitorRuleCode>,
    pub topic_id: Option<TopicId>,
    pub topic_name: Option<String>,
    pub factor_id: Option<FactorId>,
    pub factor_name: Option<String>,
    /// issue detected
    pub detected: Option<bool>,
    pub severity: Option<MonitorRuleSeverity>,
    #[serde(with = "option_naive_date")]
    pub process_date: Option<NaiveDate>,
}
