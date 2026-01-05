use crate::{ArcFactor, ArcTopic, HierarchyAid, TopicSchemaFactors};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_base::{StdR, VoidR};
use watchmen_model::{FactorId, TenantId, Topic, TopicCode, TopicData, TopicId};

/// The schema of a topic, including various factor groups.
/// all factor fields are optional, depending on whether the topic has the corresponding factors.
pub struct TopicSchema {
    topic: Arc<ArcTopic>,
    factors: Option<Arc<TopicSchemaFactors>>,
}

impl TopicSchema {
    pub fn new(topic: Topic) -> StdR<Self> {
        let arc_topic = ArcTopic::new(topic)?;
        let factors = TopicSchemaFactors::of_topic(&arc_topic)?;

        Ok(Self {
            factors: factors.if_functional(),
            topic: arc_topic,
        })
    }

    pub fn topic(&self) -> &Arc<ArcTopic> {
        &self.topic
    }

    pub fn topic_id(&self) -> &Arc<TopicId> {
        &self.topic().topic_id
    }

    pub fn name(&self) -> &Arc<TopicCode> {
        &self.topic().name
    }

    pub fn factor_by_id(&self, factor_id: &FactorId) -> Option<&ArcFactor> {
        self.topic()
            .factors
            .iter()
            .find(|f| f.factor_id.deref() == factor_id)
            .map(|f| f.deref())
    }

    pub fn factor_by_name(&self, factor_name: &String) -> Option<&ArcFactor> {
        self.topic()
            .factors
            .iter()
            .find(|f| f.name.deref() == factor_name)
            .map(|f| f.deref())
    }

    pub fn tenant_id(&self) -> &Arc<TenantId> {
        &self.topic().tenant_id
    }

    fn should_init_default_values(&self) -> bool {
        self.name().as_ref() != "raw_pipeline_monitor_log"
    }

    /// given data might be changed
    fn initialize_default_values(&self, data: &mut TopicData) {
        if self.should_init_default_values() {
            // self.default_value_factor_groups.as_deref().map(|groups| {
            //     for group in groups.iter() {
            //         group.init_default_value(data);
            //     }
            // });
        }
    }

    fn should_encrypt(&self) -> bool {
        !self.topic().kind.is_system()
    }

    /// given data might be changed
    fn encrypt(&self, data: &mut TopicData) {
        if self.should_encrypt() {
            // self.encrypt_factor_groups.as_deref().map(|groups| {
            //     for group in groups.iter() {
            //         group.encrypt(data);
            //     }
            // });
        }
    }

    /// given data might be changed
    pub fn decrypt(&self, data: &mut TopicData) {
        if self.should_encrypt() {
            // self.encrypt_factor_groups.as_deref().map(|groups| {
            //     for group in groups.iter() {
            //         group.decrypt(data);
            //     }
            // });
        }
    }

    /// given data might be changed
    fn try_cast_to_datetime(&self, data: &mut TopicData) {
        // self.date_or_time_factors.as_deref().map(|groups| {
        //     for group in groups.iter() {
        //         group.try_cast_to_datetime(data);
        //     }
        // });
    }

    /// given data might be changed
    fn flatten(&self, data: &mut TopicData) {
        if self.topic.is_raw_topic() {
            return;
        }

        // self.flatten_factors.as_deref().map(|groups| {
        //     for group in groups.iter() {
        //         group.flatten(data);
        //     }
        // });
    }

    fn should_aid_hierarchy(&self) -> bool {
        let topic = self.topic();
        !topic.is_raw_topic() && topic.name.as_ref() != "raw_pipeline_monitor_log"
    }

    /// given data might be changed
    fn aid_hierarchy(&self, data: &mut TopicData) -> VoidR {
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
    use crate::TopicSchema;
    use watchmen_model::{Factor, FactorType, Topic, TopicKind, TopicType};

    fn create_sample_topic() -> Topic {
        Topic::new()
            .topic_id(String::from("topic-1"))
            .name(String::from("Sample Topic"))
            .r#type(TopicType::Raw)
            .kind(TopicKind::Business)
            .factors(vec![
                Factor::new()
                    .factor_id("f1".to_string())
                    .name(String::from("factor-1"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("a")),
                Factor::new()
                    .factor_id("f2".to_string())
                    .name(String::from("dv.factor-2"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("b")),
                Factor::new()
                    .factor_id("f3".to_string())
                    .name(String::from("dv.factor-3"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("c")),
                Factor::new()
                    .factor_id("f4".to_string())
                    .name(String::from("dv.sub-dv.factor-4"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("d")),
                Factor::new()
                    .factor_id("f5".to_string())
                    .name(String::from("dv.sub-dv.factor-5"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("e")),
            ])
            .tenant_id(String::from("Tenant-1"))
            .version(1)
    }

    #[test]
    fn test_topic_schema() {
        let topic = create_sample_topic();
        let topic_schema = TopicSchema::new(topic).expect("failed to create topic schema");

        assert_eq!(topic_schema.topic().topic_id.as_str(), "topic-1");
        // assert!(topic_schema.default_value_factor_groups.is_none());
        // println!("{:?}", topic_schema)
    }
}
