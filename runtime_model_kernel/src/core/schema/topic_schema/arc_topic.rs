use crate::{ArcFactor, ArcHelper, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{DataSourceId, TenantId, Topic, TopicCode, TopicId, TopicKind, TopicType};
use std::sync::Arc;

/// An Arc-wrapped version of Topic for shared ownership and thread safety.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
///
/// fields are same as [Topic], description and audit columns are omitted.
#[derive(Debug)]
pub struct ArcTopic {
    pub topic_id: Arc<TopicId>,
    pub name: Arc<TopicCode>,
    pub r#type: Arc<TopicType>,
    pub kind: Arc<TopicKind>,
    pub data_source_id: Option<Arc<DataSourceId>>,
    pub factors: Arc<Vec<Arc<ArcFactor>>>,
    pub tenant_id: Arc<TenantId>,
    pub version: u32,
}

impl ArcHelper for ArcTopic {}

impl ArcTopic {
    pub fn new(topic: Topic) -> StdR<Arc<Self>> {
        let topic_id = Self::topic_id(topic.topic_id, || "Topic")?;
        let name = Self::name(topic.name, || format!("Topic[{}]", topic_id))?;
        let tenant_id = Self::tenant_id(topic.tenant_id, || format!("Topic[{}]", topic_id))?;
        let r#type = Self::must(topic.r#type, || {
            RuntimeModelKernelErrorCode::TopicTypeMissed
                .msg(format!("Topic[{}] must have a type.", topic_id))
        })?;
        let kind = Self::must(topic.kind, || {
            RuntimeModelKernelErrorCode::TopicKindMissed
                .msg(format!("Topic[{}] must have a kind.", topic_id))
        })?;
        let arc_factors = Self::must_vec(topic.factors, ArcFactor::new, || {
            RuntimeModelKernelErrorCode::TopicFactorMissed
                .msg(format!("Topic[{}] must have factor.", name))
        })?;

        Ok(Arc::new(Self {
            topic_id,
            name,
            r#type,
            kind,
            data_source_id: Self::arc(topic.data_source_id),
            factors: arc_factors,
            tenant_id,
            version: topic.version.unwrap_or(0),
        }))
    }

    pub fn is_raw_topic(&self) -> bool {
        self.r#type.is_raw_topic()
    }

    pub fn is_meta_topic(&self) -> bool {
        self.r#type.is_meta_topic()
    }

    pub fn is_distinct_topic(&self) -> bool {
        self.r#type.is_distinct_topic()
    }

    pub fn is_aggregation_topic(&self) -> bool {
        self.r#type.is_aggregation_topic()
    }

    pub fn is_time_topic(&self) -> bool {
        self.r#type.is_time_topic()
    }

    pub fn is_ratio_topic(&self) -> bool {
        self.r#type.is_ratio_topic()
    }

    pub fn is_system_topic(&self) -> bool {
        self.kind.is_system()
    }

    pub fn is_business_topic(&self) -> bool {
        self.kind.is_business()
    }

    pub fn is_synonym_topic(&self) -> bool {
        self.kind.is_synonym()
    }
}
