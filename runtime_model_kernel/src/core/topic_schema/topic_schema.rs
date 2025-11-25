use crate::{
    ArcTopic, TopicSchemaDefaultValueFactorGroup, TopicSchemaDefaultValueFactorGroups,
    TopicSchemaFactorGroups,
};
use std::sync::Arc;
use watchmen_model::{Topic, TopicData};

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    _flatten_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaFlattenFactor>>>>,
    _date_or_time_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaDateOrTimeFactor>>>>,
    _encrypt_factor_groups: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaEncryptFactor>>>>,
    default_value_factor_groups: Option<Arc<Vec<Arc<TopicSchemaDefaultValueFactorGroup>>>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> Self {
        let arc_topic = ArcTopic::from(topic);
        TopicSchema {
            topic: arc_topic.clone(),
            _flatten_factors: None,
            _date_or_time_factors: None,
            _encrypt_factor_groups: None,
            default_value_factor_groups: TopicSchemaDefaultValueFactorGroups::create(&arc_topic),
        }
    }

    pub fn topic(&self) -> &Arc<ArcTopic> {
        &self.topic
    }

    fn should_init_default_values(&self) -> bool {
        match self.topic().name.as_deref() {
            Some(name) => name.to_string() != "raw_pipeline_monitor_log",
            _ => true,
        }
    }

    /// given data might be changed, and returns exactly the given one
    pub fn initialize_default_values(&self, data: &mut TopicData) {
        if self.should_init_default_values() {
            self.default_value_factor_groups.as_deref().map(|groups| {
                for group in groups.iter() {
                    group.init_default_value(data);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use watchmen_model::{Factor, Topic, TopicKind, TopicType};

    fn create_sample_topic() -> Topic {
        Topic::new()
            .topic_id(String::from("topic-1"))
            .name(String::from("Sample Topic"))
            .r#type(TopicType::Raw)
            .kind(TopicKind::Business)
            .factors(vec![
                Factor::new()
                    .name(String::from("factor-1"))
                    .default_value(String::from("abc")),
            ])
            .tenant_id(String::from("Tenant-1"))
            .version(1)
    }

    #[test]
    fn test_topic_schema() {
        let topic = create_sample_topic();
        let topic_schema = super::TopicSchema::new(topic);

        assert_eq!(
            *topic_schema.topic().topic_id.as_deref().unwrap(),
            String::from("topic-1")
        );
        assert!(topic_schema.default_value_factor_groups.is_none());
    }
}
