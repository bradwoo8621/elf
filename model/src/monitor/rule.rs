use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, FactorId, StdErrCode, StdErrorCode, StdR, Storable, TenantBasedTuple,
    TenantId, TopicId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum MonitorRuleCode {
    // structure
    RawMismatchStructure,
    // type
    FactorMismatchEnum,
    FactorMismatchType,
    FactorMismatchDateType,
    // topic row count
    RowsNotExists,
    RowsNoChange,
    RowsCountMismatchAndAnother,
    // for all factor types
    FactorIsEmpty,
    FactorUseCast,
    FactorCommonValueOverCoverage,
    FactorEmptyOverCoverage,
    // for number type
    FactorBreaksMonotoneIncreasing,
    FactorBreaksMonotoneDecreasing,
    FactorNotInRange,
    FactorMaxNotInRange,
    FactorMinNotInRange,
    FactorAvgNotInRange,
    FactorMedianNotInRange,
    FactorQuantileNotInRange,
    FactorStdevNotInRange,
    FactorCommonValueNotInRange,
    // for string type
    FactorIsBlank,
    FactorStringLengthMismatch,
    FactorStringLengthNotInRange,
    FactorMatchRegexp,
    FactorMismatchRegexp,
    // for 2 factors
    FactorAndAnother,
}

#[derive(Display, Serde, StrEnum)]
pub enum MonitorRuleGrade {
    Global,
    Topic,
    Factor,
}

#[derive(Display, Serde, StrEnum)]
pub enum MonitorRuleSeverity {
    Fatal,
    Warn,
    Trace,
}

#[derive(Display, Serde, StrEnum)]
pub enum MonitorRuleStatisticalInterval {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Display, Serde, StrEnum)]
pub enum MonitorRuleCompareOperator {
    #[display = "eq"]
    Equal,
    #[display = "lt"]
    LessThan,
    #[display = "lte"]
    LessThanOrEqual,
    #[display = "gt"]
    GreaterThan,
    #[display = "gte"]
    GreaterThanOrEqual,
}

#[adapt_model(storable)]
pub struct MonitorRuleParameters {
    pub statistical_interval: Option<MonitorRuleStatisticalInterval>,
    pub coverage_rate: Option<i32>,
    pub aggregation: Option<i32>,
    pub quantile: Option<i32>,
    pub length: Option<i32>,
    pub max: Option<i32>,
    pub min: Option<i32>,
    pub regexp: Option<String>,
    pub compare_operator: Option<MonitorRuleCompareOperator>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

pub type MonitorRuleId = String;

#[adapt_model(tenant_based)]
pub struct MonitorRule {
    pub rule_id: Option<MonitorRuleId>,
    pub code: Option<MonitorRuleCode>,
    pub grade: Option<MonitorRuleGrade>,
    pub severity: Option<MonitorRuleSeverity>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
    pub params: Option<MonitorRuleParameters>,
    pub enabled: Option<bool>,
}
