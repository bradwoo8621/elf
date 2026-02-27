use crate::ArcTopicDataValue;
use serde::{Serialize, Serializer};
use std::sync::Arc;

pub fn serialize<S: Serializer>(
    value: &Option<Arc<ArcTopicDataValue>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => value.serialize(serializer),
        _ => serializer.serialize_none(),
    }
}
