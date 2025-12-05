use std::sync::Arc;
use watchmen_model::{StdR, TenantId, Topic, TopicCode, TopicId};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicMetaService {}

impl TopicMetaService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
    }

    pub fn find_by_id(&self, _topic_id: &TopicId, _tenant_id: &TenantId) -> StdR<Topic> {
        todo!("implement find_by_id for TopicMetaService")
    }

    pub fn find_by_code(&self, _topic_code: &TopicCode, _tenant_id: &TenantId) -> StdR<Topic> {
        todo!("implement find_by_code for TopicMetaService")
    }
}

pub trait TopicMetaProvider {
    fn meta() -> StdR<Arc<TopicMetaService>> {
        TopicMetaService::new()
    }
}
