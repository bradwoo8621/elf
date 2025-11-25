use crate::{
    ArcTopic, TopicSchemaDefaultValueFactorGroup, TopicSchemaDefaultValueFactorGroups,
    TopicSchemaFactorGroups,
};
use std::sync::Arc;
use watchmen_model::Topic;

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    flatten_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaFlattenFactor>>>>,
    date_or_time_factors: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaDateOrTimeFactor>>>>,
    encrypt_factor_groups: Option<String>, // Option<Arc<Vec<Arc<TopicSchemaEncryptFactor>>>>,
    default_value_factor_groups: Option<Arc<Vec<Arc<TopicSchemaDefaultValueFactorGroup>>>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> Self {
        let arc_topic = ArcTopic::from(topic);
        TopicSchema {
            topic: arc_topic.clone(),
            flatten_factors: None,
            date_or_time_factors: None,
            encrypt_factor_groups: None,
            default_value_factor_groups: TopicSchemaDefaultValueFactorGroups::create(&arc_topic),
        }
    }
}
