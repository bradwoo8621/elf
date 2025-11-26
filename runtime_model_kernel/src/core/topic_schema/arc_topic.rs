use crate::ArcFactor;
use std::sync::Arc;
use watchmen_model::{DataSourceId, TenantId, Topic, TopicCode, TopicId, TopicKind, TopicType};

/// An Arc-wrapped version of Topic for shared ownership and thread safety.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
///
/// fields are same as [Topic], and audit columns are omitted.
#[derive(Debug)]
pub struct ArcTopic {
    pub topic_id: Option<Arc<TopicId>>,
    pub name: Option<Arc<TopicCode>>,
    pub r#type: Option<Arc<TopicType>>,
    pub kind: Option<Arc<TopicKind>>,
    pub data_source_id: Option<Arc<DataSourceId>>,
    pub factors: Option<Arc<Vec<Arc<ArcFactor>>>>,
    pub description: Option<Arc<String>>,
    pub tenant_id: Option<Arc<TenantId>>,
    pub version: Option<u32>,
}

impl ArcTopic {
    pub fn from(topic: Topic) -> Arc<ArcTopic> {
        Arc::new(ArcTopic {
            topic_id: topic.topic_id.map(Arc::new),
            name: topic.name.map(Arc::new),
            r#type: topic.r#type.map(Arc::new),
            kind: topic.kind.map(Arc::new),
            data_source_id: topic.data_source_id.map(Arc::new),
            factors: topic
                .factors
                .map(|factors| Arc::new(factors.into_iter().map(ArcFactor::from).collect())),
            description: topic.description.map(Arc::new),
            tenant_id: topic.tenant_id.map(Arc::new),
            version: topic.version,
        })
    }

    pub fn is_raw_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_raw_topic())
            .unwrap_or(false)
    }

    pub fn is_aggregation_topic(&self) -> bool {
        self.r#type
            .as_ref()
            .map(|t| t.is_aggregation_topic())
            .unwrap_or(false)
    }
}
