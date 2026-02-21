use crate::ArcTopicDataValue;
use elf_base::StringConverter;
use elf_model::{MonitorLogDataValue, PipelineTopicData};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub struct MonitorLogHelper;

impl MonitorLogHelper {
    pub fn convert_to_log_data_map(
        topic_data: &HashMap<String, Arc<ArcTopicDataValue>>,
    ) -> PipelineTopicData {
        let mut map = HashMap::new();
        for (key, value) in topic_data.iter() {
            if let Some(converted) = Self::convert_to_log_value(value) {
                map.insert(key.clone(), converted);
            }
        }
        map
    }

    pub fn convert_to_log_value(value: &ArcTopicDataValue) -> Option<MonitorLogDataValue> {
        let converted = match value {
            ArcTopicDataValue::None => return None,
            ArcTopicDataValue::Str(str) => MonitorLogDataValue::Str(str.deref().clone()),
            ArcTopicDataValue::Num(num) => MonitorLogDataValue::Num(num.deref().clone()),
            ArcTopicDataValue::Bool(bool) => MonitorLogDataValue::Bool(*bool),
            ArcTopicDataValue::Date(date) => MonitorLogDataValue::Str(String::from_date(date)),
            ArcTopicDataValue::DateTime(datetime) => {
                MonitorLogDataValue::Str(String::from_datetime(datetime))
            }
            ArcTopicDataValue::Time(time) => MonitorLogDataValue::Str(String::from_time(time)),
            ArcTopicDataValue::Vec(vec) => {
                let mut new_vec = vec![];
                for elm in vec.iter() {
                    if let Some(new_elm) = Self::convert_to_log_value(elm) {
                        new_vec.push(new_elm);
                    }
                }
                MonitorLogDataValue::Vec(new_vec)
            }
            ArcTopicDataValue::Map(map) => {
                MonitorLogDataValue::Map(Self::convert_to_log_data_map(map.deref()))
            }
        };
        Some(converted)
    }
}
