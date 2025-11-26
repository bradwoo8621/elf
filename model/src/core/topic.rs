use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, DataSourceId, Factor, OptimisticLock, Storable, TenantBasedTuple,
    TenantId, Tuple, UserId,
};
use std::cmp::PartialEq;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, Debug, StrEnum)]
pub enum TopicKind {
    System,
    Business,
    Synonym,
}

#[derive(Display, Serde, PartialEq, Debug, StrEnum)]
pub enum TopicType {
    Raw,
    Meta,
    Distinct,
    Aggregate,
    Time,
    Ratio,
}

impl TopicType {
    pub fn is_raw_topic(&self) -> bool {
        *self == TopicType::Raw
    }

    pub fn is_aggregation_topic(&self) -> bool {
        *self == TopicType::Aggregate
    }
}

pub type TopicId = String;
pub type TopicCode = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Topic {
    pub topic_id: Option<TopicId>,
    /// must be unique within a tenant
    pub name: Option<TopicCode>,
    pub r#type: Option<TopicType>,
    pub kind: Option<TopicKind>,
    pub data_source_id: Option<DataSourceId>,
    pub factors: Option<Vec<Factor>>,
    pub description: Option<String>,
}

impl Topic {
    pub fn is_raw_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_raw_topic())
            .unwrap_or(false)
    }

    pub fn is_aggregation_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_aggregation_topic())
            .unwrap_or(false)
    }
}
