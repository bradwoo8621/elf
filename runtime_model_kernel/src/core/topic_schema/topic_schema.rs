use crate::{
    ArcTopic, TopicSchemaDefaultValueFactorGroup, TopicSchemaDefaultValueFactorGroups,
    TopicSchemaEncryptFactorGroup, TopicSchemaEncryptFactorGroups, TopicSchemaFactorGroups,
};
use std::sync::Arc;
use watchmen_model::{Topic, TopicData, TopicKind};

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
#[derive(Debug)]
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    _flatten_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaFlattenFactor>>>>,
    _date_or_time_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaDateOrTimeFactorGroup>>>>,
    encrypt_factor_groups: Option<Arc<Vec<Arc<TopicSchemaEncryptFactorGroup>>>>,
    default_value_factor_groups: Option<Arc<Vec<Arc<TopicSchemaDefaultValueFactorGroup>>>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> Self {
        let arc_topic = ArcTopic::from(topic);
        TopicSchema {
            topic: arc_topic.clone(),
            _flatten_factors: None,
            _date_or_time_factors: None,
            encrypt_factor_groups: TopicSchemaEncryptFactorGroups::create(&arc_topic),
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

    /// given data might be changed
    pub fn initialize_default_values(&self, data: &mut TopicData) {
        if self.should_init_default_values() {
            self.default_value_factor_groups.as_deref().map(|groups| {
                for group in groups.iter() {
                    group.init_default_value(data);
                }
            });
        }
    }

    fn should_encrypt(&self) -> bool {
        match self.topic().kind.as_deref().unwrap_or(&TopicKind::Business) {
            TopicKind::System => false,
            _ => true,
        }
    }

    /// given data might be changed
    pub fn encrypt(&self, data: &mut TopicData) {
        if self.should_encrypt() {
            self.encrypt_factor_groups.as_deref().map(|groups| {
                for group in groups.iter() {
                    group.encrypt(data);
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
                    .default_value(String::from("a")),
                Factor::new()
                    .name(String::from("dv.factor-2"))
                    .default_value(String::from("b")),
                Factor::new()
                    .name(String::from("dv.factor-3"))
                    .default_value(String::from("c")),
                Factor::new()
                    .name(String::from("dv.sub-dv.factor-4"))
                    .default_value(String::from("d")),
                Factor::new()
                    .name(String::from("dv.sub-dv.factor-5"))
                    .default_value(String::from("e")),
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
        // assert!(topic_schema.default_value_factor_groups.is_none());
        println!("{:?}", topic_schema)
    }
}
