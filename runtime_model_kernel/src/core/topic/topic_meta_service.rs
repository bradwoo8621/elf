use crate::TopicSchema;
use watchmen_model::{StdR, TenantId, Topic};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicMetaService {
    tenant_id: TenantId,
}

impl TopicMetaService {
    pub fn with(tenant_id: TenantId) -> Self {
        TopicMetaService { tenant_id }
    }

    pub fn find_topic(&self) -> StdR<Topic> {
        todo!("implement find_topic for TopicMetaService")
    }

    pub fn find_topic_schema(&self) -> StdR<TopicSchema> {
        Ok(TopicSchema::new(self.find_topic()?))
    }
}
