use crate::{ArcFactor, ArcTopic, TopicSchemaFactorValuePrepper, TopicSchemaFactors};
use elf_base::{StdR, VoidR};
use elf_model::{FactorId, TenantId, Topic, TopicCode, TopicData, TopicId};
use std::ops::Deref;
use std::sync::Arc;

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

    /// returns true when topic kind is not system
    fn should_encrypt(&self) -> bool {
        !self.topic().kind.is_system()
    }

    /// given data might be changed
    pub fn encrypt(&self, data: &mut TopicData) -> VoidR {
        if self.should_encrypt()
            && let Some(factors) = &self.factors
        {
            TopicSchemaFactorValuePrepper::with(
                self.topic.clone(),
                false,
                true,
                false,
                false,
                false,
            )
            .prepare(factors, data)
        } else {
            Ok(())
        }
    }

    /// given data might be changed
    pub fn decrypt(&self, data: &mut TopicData) -> VoidR {
        if self.should_encrypt()
            && let Some(factors) = &self.factors
        {
            TopicSchemaFactorValuePrepper::with(
                self.topic.clone(),
                false,
                false,
                true,
                false,
                false,
            )
            .prepare(factors, data)
        } else {
            Ok(())
        }
    }

    /// returns true when topic is not raw
    fn should_flatten(&self) -> bool {
        !self.topic.is_raw_topic()
    }

    /// returns true when topic is not raw, and not [raw_pipeline_monitor_log].
    fn should_aid_hierarchy(&self) -> bool {
        let topic = self.topic();
        !topic.is_raw_topic() && topic.name.as_ref() != "raw_pipeline_monitor_log"
    }

    /// given data might be changed
    pub fn prepare(&self, data: &mut TopicData) -> VoidR {
        if let Some(factors) = &self.factors {
            TopicSchemaFactorValuePrepper::with(
                self.topic.clone(),
                self.should_init_default_values(),
                self.should_encrypt(),
                false,
                self.should_aid_hierarchy(),
                self.should_flatten(),
            )
            .prepare(factors, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::TopicSchema;
    use chrono::Datelike;
    use elf_model::{
        Factor, FactorEncryptMethod, FactorType, Topic, TopicDataValue, TopicKind, TopicType,
    };
    use std::collections::HashMap;

    fn create_sample_topic() -> Topic {
        Topic::new()
            .topic_id(String::from("topic-1"))
            .name(String::from("Sample Topic"))
            .r#type(TopicType::Distinct)
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
                // to test prepare
                Factor::new()
                    .factor_id("f6".to_string())
                    .name(String::from("test.mask-center-3"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("abcde"))
                    .encrypt(FactorEncryptMethod::MaskCenter3),
                Factor::new()
                    .factor_id("f7".to_string())
                    .name(String::from("test.mask-last-3"))
                    .r#type(FactorType::Text)
                    .default_value(String::from("abcde"))
                    .encrypt(FactorEncryptMethod::MaskLast3),
                Factor::new()
                    .factor_id("f8".to_string())
                    .name(String::from("test.mask-month"))
                    .r#type(FactorType::Date)
                    .flatten(true)
                    .default_value(String::from("2026-02-18"))
                    .encrypt(FactorEncryptMethod::MaskMonth),
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

    #[test]
    fn test_prepare() {
        let topic = create_sample_topic();
        let topic_schema = TopicSchema::new(topic).expect("failed to create topic schema");

        let test_map = HashMap::new();
        let mut data = HashMap::new();
        data.insert("test".to_string(), TopicDataValue::Map(test_map));
        topic_schema
            .prepare(&mut data)
            .expect("failed to prepare topic schema");

        let factor_1 = data.get("factor-1").expect("failed to get factor-1");
        matches!(factor_1, TopicDataValue::Str(_));
        if let TopicDataValue::Str(s) = factor_1 {
            assert_eq!(s, "a");
        }

        let root_aid_me = data.get("aid_me").expect("failed to get aid_me from root");
        matches!(root_aid_me, TopicDataValue::Num(_));
        let root_aid_me = if let TopicDataValue::Num(s) = root_aid_me {
            s
        } else {
            panic!("failed to get aid_me from root")
        };

        // flatten
        let mask_month = data
            .get("test.mask-month")
            .expect("failed to get test.mask-month");
        matches!(mask_month, TopicDataValue::Date(_));
        let flatten_mask_month = if let TopicDataValue::Date(d) = mask_month {
            assert_eq!(d.year(), 2026);
            assert_eq!(d.month(), 1);
            assert_eq!(d.day(), 18);
            d
        } else {
            panic!("failed to get test.mask-month");
        };

        let test_map = data.get("test").expect("failed to get test topic data");
        matches!(test_map, TopicDataValue::Map(_));
        if let TopicDataValue::Map(test_map) = test_map {
            println!("test map got.");
            let mask_center_3 = test_map
                .get("mask-center-3")
                .expect("failed to get mask-center-3");
            matches!(mask_center_3, TopicDataValue::Str(_));
            if let TopicDataValue::Str(s) = mask_center_3 {
                assert_eq!(s, "a***e");
            }

            let mask_last_3 = test_map
                .get("mask-last-3")
                .expect("failed to get mask-last-3");
            matches!(mask_last_3, TopicDataValue::Str(_));
            if let TopicDataValue::Str(s) = mask_last_3 {
                assert_eq!(s, "ab***");
            }

            let mask_month = test_map
                .get("mask-month")
                .expect("failed to get mask-month");
            matches!(mask_month, TopicDataValue::Date(_));
            if let TopicDataValue::Date(d) = mask_month {
                assert_eq!(d, flatten_mask_month);
            }

            let aid_root = test_map
                .get("aid_root")
                .expect("failed to get aid_root from test map");
            matches!(aid_root, TopicDataValue::Num(_));
            if let TopicDataValue::Num(s) = aid_root {
                assert_eq!(s, root_aid_me)
            }

            let aid_me = test_map
                .get("aid_me")
                .expect("failed to get aid_me from test map");
            matches!(aid_me, TopicDataValue::Num(_));
            if let TopicDataValue::Num(s) = aid_me {
                assert_ne!(s, root_aid_me)
            }
        }
    }
}
