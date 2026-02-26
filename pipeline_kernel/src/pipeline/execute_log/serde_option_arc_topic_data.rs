use crate::ArcTopicData;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::ops::Deref;

pub fn serialize<S: Serializer>(
    value: &Option<ArcTopicData>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(value) => {
            let mut map = serializer.serialize_map(Some(value.len()))?;
            for (k, v) in value.deref() {
                map.serialize_entry(k, v.deref())?;
            }
            map.end()
        }
        _ => serializer.serialize_none(),
    }
}

// TODO deserializer of Option<ArcTopicData>
// pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ArcTopicData>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: Option<ArcTopicDataMap> = Option::deserialize(deserializer)?;
//     if let Some(s) = s {
//         Ok(Some(Arc::new(s.to_owned())))
//     } else {
//         Ok(None)
//     }
// }
