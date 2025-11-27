use crate::{
    ArcTopic, HierarchyAid, TopicSchemaDateOrTimeFactorGroup, TopicSchemaDateOrTimeFactorGroups,
    TopicSchemaDefaultValueFactorGroup, TopicSchemaDefaultValueFactorGroups,
    TopicSchemaEncryptFactorGroup, TopicSchemaEncryptFactorGroups, TopicSchemaFactorGroups,
    TopicSchemaFlattenFactorGroup, TopicSchemaFlattenFactorGroups,
};
use std::sync::Arc;
use watchmen_model::{Topic, TopicData, TopicKind, VoidR};

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
#[derive(Debug)]
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    flatten_factors: Option<Arc<Vec<Arc<TopicSchemaFlattenFactorGroup>>>>,
    date_or_time_factors: Option<Arc<Vec<Arc<TopicSchemaDateOrTimeFactorGroup>>>>,
    encrypt_factor_groups: Option<Arc<Vec<Arc<TopicSchemaEncryptFactorGroup>>>>,
    default_value_factor_groups: Option<Arc<Vec<Arc<TopicSchemaDefaultValueFactorGroup>>>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> Self {
        let arc_topic = ArcTopic::from(topic);
        TopicSchema {
            topic: arc_topic.clone(),
            flatten_factors: TopicSchemaFlattenFactorGroups::create(&arc_topic),
            date_or_time_factors: TopicSchemaDateOrTimeFactorGroups::create(&arc_topic),
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

    /// given data might be changed
    pub fn decrypt(&self, data: &mut TopicData) {
        if self.should_encrypt() {
            self.encrypt_factor_groups.as_deref().map(|groups| {
                for group in groups.iter() {
                    group.decrypt(data);
                }
            });
        }
    }

    /// given data might be changed
    pub fn try_cast_to_datetime(&self, data: &mut TopicData) {
        self.date_or_time_factors.as_deref().map(|groups| {
            for group in groups.iter() {
                group.try_cast_to_datetime(data);
            }
        });
    }

    /// given data might be changed
    pub fn flatten(&self, data: &mut TopicData) {
        if self.topic.is_raw_topic() {
            return;
        }

        self.flatten_factors.as_deref().map(|groups| {
            for group in groups.iter() {
                group.flatten(data);
            }
        });
    }

    fn should_aid_hierarchy(&self) -> bool {
        let topic = self.topic();
        if topic.is_raw_topic() {
            return false;
        }

        match topic.name.as_deref() {
            Some(name) => name.to_string() != "raw_pipeline_monitor_log",
            _ => true,
        }
    }

    /// given data might be changed
    pub fn aid_hierarchy(&self, data: &mut TopicData) -> VoidR {
        if self.should_aid_hierarchy() {
            HierarchyAid::new().aid(data)?;
        }
        Ok(())
    }

    /// given data might be changed
    pub fn prepare_data(&self, data: &mut TopicData) -> VoidR {
        self.initialize_default_values(data);
        self.try_cast_to_datetime(data);
        self.encrypt(data);
        self.aid_hierarchy(data)?;
        // flatten must be the last step
        self.flatten(data);
        Ok(())
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
