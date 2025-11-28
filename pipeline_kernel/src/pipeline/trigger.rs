use crate::{PipelineExecutionLogMonitor, TopicTrigger};
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerTraceId, PipelineTriggerType, StdR, TopicData, TopicDataId, VoidR,
};
use watchmen_runtime_model_kernel::{TopicDataService, TopicSchema};

pub struct PipelineTrigger {
    pub topic_schema: Arc<TopicSchema>,
    pub r#type: PipelineTriggerType,
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: PipelineExecutionLogMonitor,
}

impl PipelineTrigger {
    fn prepare_trigger_data(&self, data: &mut TopicData) -> VoidR {
        self.topic_schema.prepare_data(data)
    }

    fn find_topic_data_service(&self) -> StdR<Arc<TopicDataService>> {
        TopicDataService::with(&self.principal.tenant_id)
    }

    fn save_trigger_data(&self, mut data: TopicData) -> StdR<Arc<TopicTrigger>> {
        let topic = self.topic_schema.topic();

        self.prepare_trigger_data(&mut data)?;

        if topic.is_synonym_topic() && self.r#type.is_insert() {
            TopicTrigger::insert_to_synonym(data)
        } else {
            let topic_data_service = self.find_topic_data_service()?;

            match self.r#type {
                PipelineTriggerType::Insert => {
                    let current_data =
                        topic_data_service.insert(&self.topic_schema.topic_name(), data)?;
                    TopicTrigger::insert(current_data)
                }
                PipelineTriggerType::InsertOrMerge => {
                    let (previous_data, current_data) = topic_data_service
                        .insert_or_merge(&self.topic_schema.topic_name(), data)?;
                    match previous_data {
                        Some(previous_data) => TopicTrigger::merge(previous_data, current_data),
                        _ => TopicTrigger::insert(current_data),
                    }
                }
                PipelineTriggerType::Merge => {
                    let (previous_data, current_data) =
                        topic_data_service.merge(&self.topic_schema.topic_name(), data)?;
                    TopicTrigger::merge(previous_data, current_data)
                }
                PipelineTriggerType::Delete => {
                    let previous_data =
                        topic_data_service.delete(&self.topic_schema.topic_name(), data)?;
                    TopicTrigger::delete(previous_data)
                }
            }
        }
    }

    pub fn execute(&self, data: TopicData) -> StdR<TopicDataId> {
        let topic_trigger = self.save_trigger_data(data)?;
        Ok(topic_trigger.data_id())
    }

    pub async fn execute_async(&self, data: TopicData) -> StdR<TopicDataId> {
        let topic_trigger = self.save_trigger_data(data)?;
        Ok(topic_trigger.data_id())
    }
}
