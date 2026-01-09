use crate::{ArcTopicData, ArcTopicDataMap, ArcTopicDataValue};
use elf_model::{TopicData, TopicDataValue};
use std::collections::HashMap;
use std::sync::Arc;

pub trait ArcTopicDataBuilder {
    fn build_value(value: TopicDataValue) -> Arc<ArcTopicDataValue> {
        Arc::new(match value {
            TopicDataValue::Str(str) => ArcTopicDataValue::Str(Arc::new(str)),
            TopicDataValue::Num(num) => ArcTopicDataValue::Num(Arc::new(num)),
            TopicDataValue::Bool(bool) => ArcTopicDataValue::Bool(bool),
            TopicDataValue::DateTime(datetime) => ArcTopicDataValue::DateTime(Arc::new(datetime)),
            TopicDataValue::Date(date) => ArcTopicDataValue::Date(Arc::new(date)),
            TopicDataValue::Time(time) => ArcTopicDataValue::Time(Arc::new(time)),
            TopicDataValue::Vec(vec) => ArcTopicDataValue::Vec(Arc::new(Self::build_vec(vec))),
            TopicDataValue::Map(map) => ArcTopicDataValue::Map(Arc::new(Self::build_map(map))),
            TopicDataValue::None => ArcTopicDataValue::None,
        })
    }

    fn build_vec(vec: Vec<TopicDataValue>) -> Vec<Arc<ArcTopicDataValue>> {
        vec.into_iter().map(|v| Self::build_value(v)).collect()
    }

    fn build_map(map: HashMap<String, TopicDataValue>) -> ArcTopicDataMap {
        map.into_iter()
            .map(|(k, v)| (k, Self::build_value(v)))
            .collect()
    }

    fn build(data: TopicData) -> ArcTopicData {
        let mut arc_data = HashMap::with_capacity(data.len());

        data.into_iter().for_each(|(key, value)| {
            arc_data.insert(key, Self::build_value(value));
        });

        Arc::new(arc_data)
    }
}

impl ArcTopicDataBuilder for ArcTopicData {}
