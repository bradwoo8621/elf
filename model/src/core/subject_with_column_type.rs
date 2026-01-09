use crate::{
	Auditable, BaseDataModel, ConnectedSpaceId, FactorType, LastVisit, Parameter, ParameterJoint,
	Storable, SubjectColumnArithmetic, SubjectDataSetColumnRenderer, SubjectDatasetColumnId,
	SubjectDatasetJoin, SubjectId, TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

/// extend a [column_type] field from [SubjectDatasetColumn]
#[adapt_model(storable)]
pub struct SubjectDatasetColumnWithType {
    pub column_id: Option<SubjectDatasetColumnId>,
    pub parameter: Option<Parameter>,
    pub alias: Option<String>,
    pub arithmetic: Option<SubjectColumnArithmetic>,
    pub renderer: Option<SubjectDataSetColumnRenderer>,
    /// recalculated column based on other columns
    /// source of this column must be referred to other column, via columnId or alias
    /// if column is declared as recalculate, arithmetic will be ignored
    /// and refer to another recalculated column is not allowed
    pub recalculate: Option<bool>,
    pub column_type: Option<FactorType>,
}

/// fields are same as [SubjectDataset],
/// use [SubjectDatasetColumnWithType] as type of [columns], instead of [SubjectDatasetColumn].
#[adapt_model(storable)]
pub struct SubjectDatasetWithColumnType {
    pub columns: Option<Vec<SubjectDatasetColumnWithType>>,
    pub joins: Option<Vec<SubjectDatasetJoin>>,
    pub filters: Option<ParameterJoint>,
}

/// fields are same as [Subject],
/// use [SubjectDatasetWithColumnType] as type of [dataset], instead of [SubjectDataset].
#[adapt_model(user_based, audit, last_visit)]
pub struct SubjectWitColumnType {
    pub subject_id: Option<SubjectId>,
    pub name: Option<String>,
    pub connect_id: Option<ConnectedSpaceId>,
    pub auto_refresh_interval: Option<i32>,
    pub dataset: Option<SubjectDatasetWithColumnType>,
}
