use crate::{TopicMetaProvider, TopicSchema};
use std::sync::Arc;
use watchmen_base::StdR;
use watchmen_model::TopicData;

pub struct TopicDataService;

impl TopicMetaProvider for TopicDataService {}

impl TopicDataService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
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

pub trait TopicDataProvider {
    fn data() -> StdR<Arc<TopicDataService>> {
        TopicDataService::new()
    }
}
