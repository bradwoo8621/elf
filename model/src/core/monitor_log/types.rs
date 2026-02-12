use bigdecimal::BigDecimal;
use elf_model_marco::VariousValueTypes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MonitorLogActionId = String;

#[derive(Deserialize, Serialize, Clone, Debug, VariousValueTypes)]
#[serde(untagged)]
pub enum DataOnMonitorLog {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    Vec(Vec<DataOnMonitorLog>),
    Map(HashMap<String, DataOnMonitorLog>),
}

pub type MapDataOnMonitorLog = HashMap<String, DataOnMonitorLog>;

/// TODO Any needs to be changed to some struct, according to where it is
///  there sure thing is, it is not a [String]
pub type NotKnownYetDataStruct = String;

// prerequisite definition
pub type PrerequisiteDefinedAs = MapDataOnMonitorLog;

// unit loop variable value
pub type UnitLoopVariableValue = DataOnMonitorLog;

// action definition
pub type ActionDefinedAs = MapDataOnMonitorLog;
// action data find by, runtime criteria
pub type ActionFindByCriteria = MapDataOnMonitorLog;
/// action touched value,
/// for deletion, update and insert, always be list of dict
/// for read-exists, bool,
/// for read-factor, no arithmetic, Any, depends on factor type
/// for read-factor, arithmetic, Decimal
/// for read-row, dict
/// for read-rows, list of dict
pub type ActionTouchedValues = DataOnMonitorLog;
