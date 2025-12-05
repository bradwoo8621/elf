use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, BucketId, FactorOrSubjectDatasetColumnId, OptimisticLock,
    ParameterJoint, StdErrCode, StdErrorCode, StdR, Storable, TenantBasedTuple, TenantId,
    TopicOrSubjectId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum IndicatorAggregateArithmetic {
    Count,
    #[display = "distinct_count"]
    DistinctCount,
    Sum,
    Avg,
    Max,
    Min,
}

#[derive(Display, Serde, StrEnum)]
pub enum RelevantIndicatorType {
    Same,
    HighCorrelated,
    WeakCorrelated,
    ThisCauseRelevant,
    RelevantCausesThis,
}

pub type IndicatorId = String;

#[adapt_model(storable)]
pub struct RelevantIndicator {
    pub indicator_id: Option<IndicatorId>,
    pub r#type: Option<RelevantIndicatorType>,
}

#[derive(Display, Serde, StrEnum)]
pub enum IndicatorBaseOn {
    Topic,
    Subject,
}

#[adapt_model(storable)]
pub struct IndicatorFilter {
    pub enabled: Option<bool>,
    pub joint: Option<ParameterJoint>,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct Indicator {
    pub indicator_id: Option<IndicatorId>,
    pub name: Option<String>,
    /// when indicator is on topic
    pub topic_or_subject_id: Option<TopicOrSubjectId>,
    /// is a count indicator when factor is not declared
    /// it is columnId when base one a subject
    pub factor_id: Option<FactorOrSubjectDatasetColumnId>,
    /// only count can be applied when factor id is not declared
    pub aggregate_arithmetic: Option<IndicatorAggregateArithmetic>,
    pub base_on: Option<IndicatorBaseOn>,
    pub category1: Option<String>,
    pub category2: Option<String>,
    pub category3: Option<String>,
    pub description: Option<String>,
    /// effective only when factorId is appointed
    pub value_buckets: Option<Vec<BucketId>>,
    /// noinspection SpellCheckingInspection
    pub relevants: Option<Vec<RelevantIndicator>>,
    pub filter: Option<IndicatorFilter>,
}
