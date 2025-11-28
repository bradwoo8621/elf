use crate::{PipelineExecutionLogMonitor, PipelineKernelErrorCode, TopicTrigger};
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineId, PipelineTriggerTraceId, PipelineTriggerType, StdErrorCode, StdR, TopicData,
    TopicDataId, VoidR,
};
use watchmen_runtime_model_kernel::{PipelineMetaService, TopicDataService, TopicSchema};

pub struct PipelineTrigger {
    pub pipeline_id: Option<PipelineId>,
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
                    let current_data = topic_data_service.insert(&self.topic_schema, data)?;
                    TopicTrigger::insert(current_data)
                }
                PipelineTriggerType::InsertOrMerge => {
                    let (previous_data, current_data) =
                        topic_data_service.insert_or_merge(&self.topic_schema, data)?;
                    match previous_data {
                        Some(previous_data) => TopicTrigger::merge(previous_data, current_data),
                        _ => TopicTrigger::insert(current_data),
                    }
                }
                PipelineTriggerType::Merge => {
                    let (previous_data, current_data) =
                        topic_data_service.merge(&self.topic_schema, data)?;
                    TopicTrigger::merge(previous_data, current_data)
                }
                PipelineTriggerType::Delete => {
                    let previous_data = topic_data_service.delete(&self.topic_schema, data)?;
                    TopicTrigger::delete(previous_data)
                }
            }
        }
    }

    fn find_pipeline_meta_service(&self) -> StdR<Arc<PipelineMetaService>> {
        PipelineMetaService::with(&self.principal.tenant_id)
    }


    /// TODO
    fn prepare_executor(self) -> StdR<String> {
        let pipeline_meta_service = self.find_pipeline_meta_service()?;
        let pipelines = match self.pipeline_id {
            Some(pipeline_id) => {
                let pipeline = pipeline_meta_service.find_by_id(&pipeline_id)?;
                if let Some(pipeline) = pipeline {
                    return PipelineKernelErrorCode::TriggerTypeMismatchPipeline.msg(format!(
                        "Trigger type[{}] does not match pipeline[]",
                        &self.r#type
                    ));
                    vec![pipeline]
                } else {
                    return PipelineKernelErrorCode::TriggerPipelineNotFound
                        .msg(format!("Trigger pipeline[{}] not found.", &pipeline_id));
                }
            }
            _ => pipeline_meta_service.find_by_topic_and_pipeline_type(
                self.topic_schema.topic().topic_id.as_ref(),
                &self.r#type,
            )?,
        };

        Ok("".to_string())
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
