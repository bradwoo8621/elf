use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, ExternalWriterId, FactorId, OptimisticLock, Parameter,
    ParameterJoint, Storable, TenantBasedTuple, TenantId, TopicId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
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

pub type PipelineActionId = String;

pub trait IPipelineAction {
    fn action_id() -> Option<PipelineActionId>;
    fn r#type() -> Option<PipelineActionType>;
}

pub trait MemoryWriter: IPipelineAction {
    fn variable_name() -> Option<String>;
}

pub trait FromTopic: IPipelineAction {
    fn topic_id() -> Option<TopicId>;
}

pub trait FromFactor: FromTopic {
    fn factor_id() -> Option<FactorId>;
}

pub trait ToTopic: IPipelineAction {
    fn topic_id() -> Option<TopicId>;
}

pub trait ToFactor: ToTopic {
    fn factor_id() -> Option<FactorId>;
}

pub trait FindBy: IPipelineAction {
    fn by() -> Option<ParameterJoint>;
}

#[derive(Display, Serde)]
pub enum AlarmActionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[adapt_model(storable)]
pub struct AlarmAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub severity: Option<AlarmActionSeverity>,
    pub message: Option<String>,
    pub conditional: Option<bool>,
    pub on: Option<ParameterJoint>,
}

/// copy something to memory variable
#[adapt_model(storable)]
pub struct CopyToMemoryAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub source: Option<Parameter>,
    pub variable_name: Option<String>,
}

#[adapt_model(storable)]
pub struct WriteToExternalAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub external_writer_id: Option<ExternalWriterId>,
    pub event_code: Option<String>,
}

#[adapt_model(storable)]
pub struct ReadRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

#[adapt_model(storable)]
pub struct ReadRowsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

#[derive(Display, Serde)]
pub enum AggregateArithmetic {
    None,
    Count,
    Sum,
    Avg,
}

#[adapt_model(storable)]
pub struct ReadFactorAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
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

#[adapt_model(storable)]
pub struct ReadFactorsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read from factor
    pub factor_id: Option<FactorId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

#[adapt_model(storable)]
pub struct ExistsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// read from topic
    pub topic_id: Option<TopicId>,
    /// read criteria
    pub by: Option<ParameterJoint>,
    /// copy to memory variable
    pub variable_name: Option<String>,
}

#[adapt_model(storable)]
pub struct MappingFactor {
    pub source: Option<Parameter>,
    pub factor_id: Option<FactorId>,
    pub arithmetic: Option<AggregateArithmetic>,
}

#[derive(Display, Serde)]
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
    /// WriteTopicAction, MappingRow
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
}

#[adapt_model(storable)]
pub struct InsertOrMergeRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
    /// write criteria
    pub by: Option<ParameterJoint>,
}

#[adapt_model(storable)]
pub struct MergeRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    pub accumulate_mode: Option<AccumulateMode>,
    pub mapping: Option<Vec<MappingFactor>>,
    /// write to topic
    pub topic_id: Option<TopicId>,
    /// write criteria
    pub by: Option<ParameterJoint>,
}

#[adapt_model(storable)]
pub struct WriteFactorAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
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

#[adapt_model(storable)]
pub struct DeleteRowAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// delete from topic
    pub topic_id: Option<TopicId>,
    /// delete criteria
    pub by: Option<ParameterJoint>,
}

#[adapt_model(storable)]
pub struct DeleteRowsAction {
    pub action_id: Option<PipelineActionId>,
    pub r#type: PipelineActionType,
    /// delete from topic
    pub topic_id: Option<TopicId>,
    /// delete criteria
    pub by: Option<ParameterJoint>,
}

#[adapt_model(storable)]
pub enum PipelineAction {
    Alarm(AlarmAction),
    CopyToMemory(CopyToMemoryAction),
    WriteToExternal(WriteToExternalAction),
    ReadRow(ReadRowAction),
    ReadFactor(ReadFactorAction),
    Exists(ExistsAction),
    ReadRows(ReadRowsAction),
    ReadFactors(ReadFactorsAction),
    MergeRow(MergeRowAction),
    InsertRow(InsertRowAction),
    InsertOrMergeRow(InsertOrMergeRowAction),
    WriteFactor(WriteFactorAction),
    DeleteRow(DeleteRowAction),
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

#[derive(Display, Serde)]
pub enum PipelineTriggerType {
    Insert,
    Merge,
    InsertOrMerge,
    Delete,
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
