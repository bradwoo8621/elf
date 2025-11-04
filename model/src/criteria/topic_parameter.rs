use super::{
    parameter::{ConditionalParameter, Parameter},
    parameter_joint::ParameterJoint,
    parameter_kind::ParameterKind,
};
use crate::common::base::DataModelValue;
use crate::common::{
    base::BaseDataModel,
    tuple_ids::{FactorId, TopicId},
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct TopicFactorParameter {
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

impl BaseDataModel for TopicFactorParameter {
    fn to_map(&self) -> HashMap<&str, DataModelValue> {
        let mut map = HashMap::new();
        map.insert("kind", json!(ParameterKind::Topic));
        if let Some(topic_id) = &self.topic_id {
            map.insert("topicId", Value::from(topic_id.clone()));
        }
        if let Some(factor_id) = &self.factor_id {
            map.insert("factorId", Value::from(factor_id.clone()));
        }
        map
    }
}

impl Parameter for TopicFactorParameter {
    fn kind(&self) -> ParameterKind {
        ParameterKind::Topic
    }
}

impl ConditionalParameter for TopicFactorParameter {
    fn conditional(&self) -> Option<bool> {
        todo!("Not implemented yet")
    }

    fn on(&self) -> Option<Box<dyn ParameterJoint>> {
        todo!("Not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_map_minimal() {
        let p = TopicFactorParameter {
            topic_id: Some("10001".to_string()),
            factor_id: Some("20001".to_string()),
        };
        let map = p.to_map();
        // assert_eq!(map.get("kind"), Some(&json!(ParameterKind::Topic)));
        // assert!(!map.contains_key("topicId"));
        // assert!(!map.contains_key("factorId"));
        println!("{}", serde_json::to_string(&map).unwrap());
    }
}
