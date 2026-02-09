pub type MonitorLogActionId = String;

/// TODO Any needs to be changed to some struct, according to where it is
///  there sure thing is, it is not a [String]
pub type NotKnownYetDataStruct = String;

// prerequisite definition
pub type PrerequisiteDefinedAs = NotKnownYetDataStruct;

// action definition
pub type ActionDefinedAs = NotKnownYetDataStruct;
// action data find by, runtime criteria
pub type ActionFindByCriteria = NotKnownYetDataStruct;
/// action touched value,
/// for deletion, update and insert, always be list of dict
/// for read-exists, bool,
/// for read-factor, no arithmetic, Any, depends on factor type
/// for read-factor, arithmetic, Decimal
/// for read-row, dict
/// for read-rows, list of dict
pub type ActionTouchedValues = NotKnownYetDataStruct;
