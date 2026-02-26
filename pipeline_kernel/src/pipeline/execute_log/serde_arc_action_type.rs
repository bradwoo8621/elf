use elf_model::PipelineActionType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

pub fn serialize<S: Serializer>(
    value: &Arc<PipelineActionType>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value.serialize(serializer)
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<PipelineActionType>, D::Error>
where
    D: Deserializer<'de>,
{
    PipelineActionType::deserialize(deserializer).map(Arc::new)
}
