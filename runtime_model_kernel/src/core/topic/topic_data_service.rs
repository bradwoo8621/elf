use crate::{TopicMetaService, TopicSchema};
use std::sync::Arc;
use watchmen_model::{StdR, TenantId, TopicData};

pub struct TopicDataService {
    tenant_id: TenantId,
    meta: Arc<TopicMetaService>,
}

impl TopicDataService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(TopicDataService {
            tenant_id: tenant_id.clone(),
            meta: TopicMetaService::with(&tenant_id)?,
        }))
    }

    pub fn insert(&self, _topic_schema: &Arc<TopicSchema>, _data: TopicData) -> StdR<TopicData> {
        todo!("implement insert for TopicDataService")
    }

    pub fn insert_or_merge(
        &self,
        _topic_schema: &Arc<TopicSchema>,
        _data: TopicData,
    ) -> StdR<(Option<TopicData>, TopicData)> {
        todo!("implement insert_or_merge for TopicDataService")
    }

    pub fn merge(
        &self,
        _topic_schema: &Arc<TopicSchema>,
        _data: TopicData,
    ) -> StdR<(TopicData, TopicData)> {
        todo!("implement merge for TopicDataService")
    }

    pub fn delete(&self, _topic_schema: &Arc<TopicSchema>, _data: TopicData) -> StdR<TopicData> {
        todo!("implement delete for TopicDataService")
    }
}
