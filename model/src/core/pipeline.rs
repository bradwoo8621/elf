use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, ExternalWriterId, FactorId, OptimisticLock, Parameter,
    ParameterJoint, StdErrCode, StdErrorCode, StdR, Storable, TenantBasedTuple, TenantId, TopicId,
    Tuple, UserId,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, Debug, StrEnum)]
pub enum PipelineActionType {
    // system actions
    Alarm,
    CopyToMemory,
    WriteToExternal,
    // read topic actions
    ReadRow,
    ReadFactor,
    Exists,
    ReadRows,
    ReadFactors,
    // write topic actions
    MergeRow,
    InsertRow,
    InsertOrMergeRow,
    WriteFactor,
    // delete topic actions
    DeleteRow,
    DeleteRows,
}

impl PipelineActionType {
    pub fn for_read(self) -> bool {
        match self {
            PipelineActionType::ReadRow => true,
            PipelineActionType::ReadFactor => true,
            PipelineActionType::Exists => true,
            PipelineActionType::ReadRows => true,
            PipelineActionType::ReadFactors => true,
            _ => false,
        }
    }

    pub fn for_write(self) -> bool {
        match self {
            PipelineActionType::InsertRow => true,
            PipelineActionType::MergeRow => true,
            PipelineActionType::InsertOrMergeRow => true,
            PipelineActionType::WriteFactor => true,
            _ => false,
        }
    }

    pub fn for_delete(self) -> bool {
        match self {
            PipelineActionType::DeleteRow => true,
            PipelineActionType::DeleteRows => true,
            _ => false,
        }
    }
}

pub type PipelineActionId = String;

#[derive(Display, Serde, Debug, StrEnum)]
pub enum AlarmActionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[adapt_model(storable)]
pub struct AlarmAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub severity: Option<AlarmActionSeverity>,
    pub message: Option<String>,
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
}

impl AlarmAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::Alarm)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::Alarm(self)
    }
}

/// copy something to memory variable
#[adapt_model(storable)]
pub struct CopyToMemoryAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub source: Option<Parameter>,
    pub variable_name: Option<String>,
}

impl CopyToMemoryAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::CopyToMemory)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::CopyToMemory(self)
    }
}

#[adapt_model(storable)]
pub struct WriteToExternalAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub external_writer_id: Option<ExternalWriterId>,
    pub event_code: Option<String>,
}

impl WriteToExternalAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::WriteToExternal)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::WriteToExternal(self)
    }
}

#[adapt_model(storable)]
pub struct ReadRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

impl ReadRowAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::ReadRow)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::ReadRow(self)
    }
}

#[adapt_model(storable)]
pub struct ReadRowsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

impl ReadRowsAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::ReadRows)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::ReadRows(self)
    }
}

#[derive(Display, Serde, Debug, StrEnum)]
pub enum AggregateArithmetic {
    None,
    Count,
    Sum,
    Avg,
}

#[adapt_model(storable)]
pub struct ReadFactorAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read from factor
    pub factor_id: Option<FactorId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
    pub arithmetic: Option<AggregateArithmetic>,
}

impl ReadFactorAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::ReadFactor)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::ReadFactor(self)
    }
}

#[adapt_model(storable)]
pub struct ReadFactorsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read from factor
    pub factor_id: Option<FactorId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

impl ReadFactorsAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::ReadFactors)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::ReadFactors(self)
    }
}

#[adapt_model(storable)]
pub struct ExistsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

impl ExistsAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::Exists)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::Exists(self)
    }
}

#[adapt_model(storable)]
pub struct MappingFactor {
    pub source: Option<Parameter>,
    pub factor_id: Option<FactorId>,
    pub arithmetic: Option<AggregateArithmetic>,
}

impl MappingFactor {
    pub fn direct() -> Self {
        Self::new().arithmetic(AggregateArithmetic::None)
    }

    pub fn sum() -> Self {
        Self::new().arithmetic(AggregateArithmetic::Sum)
    }

    pub fn avg() -> Self {
        Self::new().arithmetic(AggregateArithmetic::Avg)
    }

    pub fn count() -> Self {
        Self::new().arithmetic(AggregateArithmetic::Count)
    }
}

#[derive(Display, Serde, Debug, StrEnum)]
pub enum AccumulateMode {
    /// add value in current data for insert
    /// subtract value in previous data, add value in current data for merge
    Standard,
    /// allowed only on explicit merge action (merge row/write factor)
    /// subtract value in previous data
    Reverse,
    /// force cumulate, not matter there is previous or not. always accumulate to existing value
    /// not allowed on insert action. actually for explicit insert action, behaviour is same as standard mode
    /// ignore previous data even existing, add value in current data only
    Cumulate,
}

#[adapt_model(storable)]
pub struct InsertRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
}

impl InsertRowAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::InsertRow)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::InsertRow(self)
    }
}

// noinspection DuplicatedCode
#[adapt_model(storable)]
pub struct InsertOrMergeRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
    /// write criteria
    pub by: Option<ParameterJoint>,
}

impl InsertOrMergeRowAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::InsertOrMergeRow)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::InsertOrMergeRow(self)
    }
}

// noinspection DuplicatedCode
#[adapt_model(storable)]
pub struct MergeRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
    /// write criteria
    pub by: Option<ParameterJoint>,
}

impl MergeRowAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::MergeRow)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::MergeRow(self)
    }
}

#[adapt_model(storable)]
pub struct WriteFactorAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    pub accumulate_mode: Option<AccumulateMode>,
    pub source: Option<Parameter>,
    /// write to topic
    pub topic_id: Option<TopicId>,
    /// write to factor
    pub factor_id: Option<FactorId>,
    /// write criteria
    pub by: Option<ParameterJoint>,
    pub arithmetic: Option<AggregateArithmetic>,
}

impl WriteFactorAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::WriteFactor)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::WriteFactor(self)
    }
}

#[adapt_model(storable)]
pub struct DeleteRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// delete from topic
    pub topic_id: Option<TopicId>,
    /// delete criteria
    pub by: Option<ParameterJoint>,
}

impl DeleteRowAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::DeleteRow)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::DeleteRow(self)
    }
}

#[adapt_model(storable)]
pub struct DeleteRowsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: Option<PipelineActionType>,
    /// delete from topic
    pub topic_id: Option<TopicId>,
    /// delete criteria
    pub by: Option<ParameterJoint>,
}

impl DeleteRowsAction {
    pub fn init() -> Self {
        Self::new().r#type(PipelineActionType::DeleteRows)
    }

    pub fn to_action(self) -> PipelineAction {
        PipelineAction::DeleteRows(self)
    }
}

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "type")]
pub enum PipelineAction {
    #[serde(rename = "alarm")]
    Alarm(AlarmAction),
    #[serde(rename = "copy-to-memory")]
    CopyToMemory(CopyToMemoryAction),
    #[serde(rename = "write-to-external")]
    WriteToExternal(WriteToExternalAction),
    #[serde(rename = "read-row")]
    ReadRow(ReadRowAction),
    #[serde(rename = "read-factor")]
    ReadFactor(ReadFactorAction),
    #[serde(rename = "exists")]
    Exists(ExistsAction),
    #[serde(rename = "read-rows")]
    ReadRows(ReadRowsAction),
    #[serde(rename = "read-factors")]
    ReadFactors(ReadFactorsAction),
    #[serde(rename = "merge-row")]
    MergeRow(MergeRowAction),
    #[serde(rename = "insert-row")]
    InsertRow(InsertRowAction),
    #[serde(rename = "insert-or-merge-row")]
    InsertOrMergeRow(InsertOrMergeRowAction),
    #[serde(rename = "write-factor")]
    WriteFactor(WriteFactorAction),
    #[serde(rename = "delete-row")]
    DeleteRow(DeleteRowAction),
    #[serde(rename = "delete-rows")]
    DeleteRows(DeleteRowsAction),
}

pub type PipelineUnitId = String;

#[adapt_model(storable)]
pub struct PipelineUnit {
    pub unit_id: Option<PipelineUnitId>,
    pub name: Option<String>,
    pub loop_variable_name: Option<String>,
    pub r#do: Option<Vec<PipelineAction>>,
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
}

pub type PipelineStageId = String;

#[adapt_model(storable)]
pub struct PipelineStage {
    pub stage_id: Option<PipelineStageId>,
    pub name: Option<String>,
    pub units: Option<Vec<PipelineUnit>>,
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
}

#[derive(Display, Serde, PartialEq, Debug, StrEnum)]
pub enum PipelineTriggerType {
    Insert,
    Merge,
    InsertOrMerge,
    Delete,
}

impl PipelineTriggerType {
    pub fn is_insert(&self) -> bool {
        *self == PipelineTriggerType::Insert
    }

    pub fn is_insert_or_merge(&self) -> bool {
        *self == PipelineTriggerType::InsertOrMerge
    }

    pub fn is_merge(&self) -> bool {
        *self == PipelineTriggerType::Merge
    }

    pub fn is_delete(&self) -> bool {
        *self == PipelineTriggerType::Delete
    }
}

pub type PipelineId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Pipeline {
    pub pipeline_id: Option<PipelineId>,
    pub topic_id: Option<TopicId>,
    pub name: Option<String>,
    pub r#type: Option<PipelineTriggerType>,
    pub stages: Option<Vec<PipelineStage>>,
    pub enabled: Option<bool>,
    pub validated: Option<bool>,
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
}
