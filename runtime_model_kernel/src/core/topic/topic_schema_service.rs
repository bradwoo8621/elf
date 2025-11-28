use crate::{TopicMetaService, TopicSchema};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId, TopicCode};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicSchemaService {
    tenant_id: TenantId,
    meta: Arc<TopicMetaService>,
}

// TODO maybe find from cache
impl TopicSchemaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        Ok(Arc::new(TopicSchemaService {
            tenant_id: tenant_id.clone(),
            meta: TopicMetaService::with(tenant_id)?,
        }))
    }

    pub fn find_by_code(&self, code: &TopicCode) -> StdR<Arc<TopicSchema>> {
        let topic = self.meta.find_by_code(code)?;
        let schema = TopicSchema::new(topic)?;
        Ok(Arc::new(schema))
    }
}
