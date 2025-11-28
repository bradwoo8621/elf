use crate::{ArcFactor, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    DataSourceId, StdErrorCode, StdR, TenantId, Topic, TopicCode, TopicId, TopicKind, TopicType,
};

/// An Arc-wrapped version of Topic for shared ownership and thread safety.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
///
/// fields are same as [Topic], description and audit columns are omitted.
#[derive(Debug)]
pub struct ArcTopic {
    pub topic_id: Option<Arc<TopicId>>,
    pub name: Arc<TopicCode>,
    pub r#type: Arc<TopicType>,
    pub kind: Arc<TopicKind>,
    pub data_source_id: Option<Arc<DataSourceId>>,
    pub factors: Option<Arc<Vec<Arc<ArcFactor>>>>,
    pub tenant_id: Arc<TenantId>,
    pub version: Option<u32>,
}

impl ArcTopic {
    pub fn from(topic: Topic) -> StdR<Arc<ArcTopic>> {
        if topic.name.is_none() {
            return RuntimeModelKernelErrorCode::TopicNameMissed.msg("Topic must have a name.");
        }
        let name = Arc::new(topic.name.unwrap());

        if topic.tenant_id.is_none() {
            return RuntimeModelKernelErrorCode::TopicNameMissed
                .msg(format!("Topic[{}] has not tenant.", name));
        }
        let tenant_id = Arc::new(topic.tenant_id.unwrap());

        if topic.r#type.is_none() {
            return RuntimeModelKernelErrorCode::TopicTypeMissed
                .msg(format!("Topic[{}] has no type.", name));
        }
        if topic.kind.is_none() {
            return RuntimeModelKernelErrorCode::TopicKindMissed
                .msg(format!("Topic[{}] has no kind.", name));
        }

        Ok(Arc::new(ArcTopic {
            topic_id: topic.topic_id.map(Arc::new),
            name,
            r#type: Arc::new(topic.r#type.unwrap()),
            kind: Arc::new(topic.kind.unwrap()),
            data_source_id: topic.data_source_id.map(Arc::new),
            factors: topic
                .factors
                .map(|factors| Arc::new(factors.into_iter().map(ArcFactor::from).collect())),
            tenant_id,
            version: topic.version,
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
