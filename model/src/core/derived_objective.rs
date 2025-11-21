use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, BucketId, FactorOrSubjectDatasetColumnId, LastVisit, MeasureMethod,
    Objective, ObjectiveId, ObjectiveTargetId, Storable, TenantId, UserBasedTuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum BreakdownDimensionType {
    Value,
    Bucket,
    #[display = "time"]
    TimeRelated,
}

/// when type is VALUE, which means no bucket, no time measure method. use the original value as dimension
#[adapt_model(storable)]
pub struct BreakdownDimension {
    pub r#type: Option<BreakdownDimensionType>,
    /// if measure on factor,
    /// factor id must be given
    pub factor_or_column_id: Option<FactorOrSubjectDatasetColumnId>,
    /// bucket for any measure on type
    pub bucket_id: Option<BucketId>,
    /// only when factor/column is date,
    /// and adaptable time measure method could be applied
    /// for example,
    /// if factor is date,
    /// then YEAR/QUARTER/MONTH/etc. could be applied to it
    /// if factor is year,
    /// then only YEAR could be applied to it.
    pub time_measure_method: Option<MeasureMethod>,
}

pub type BreakdownTargetId = String;

#[adapt_model(storable)]
pub struct BreakdownTarget {
    pub uuid: Option<BreakdownTargetId>,
    pub target_id: Option<ObjectiveTargetId>,
    pub name: Option<String>,
    pub dimensions: Option<Vec<BreakdownDimension>>,
}

pub type DerivedObjectiveId = String;

#[adapt_model(user_based, audit, last_visit)]
pub struct DerivedObjective {
    pub derived_objective_id: Option<DerivedObjectiveId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub objective_id: Option<ObjectiveId>,
    pub definition: Option<Objective>,
    pub breakdown_targets: Option<Vec<BreakdownTarget>>,
}
