use crate::{
    PipelineExecutionLogMonitor, PipelineKernelErrorCode, PipelineRunContext, PipelineRunner,
    TopicTrigger,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineId, PipelineTriggerTraceId, PipelineTriggerType, StdErrorCode, StdR, TopicData,
    TopicDataId, VoidR,
};
use watchmen_runtime_model_kernel::{
    PipelineSchema, PipelineSchemaService, TopicDataService, TopicSchema,
};

pub struct PipelineTrigger {
    pub pipeline_id: Option<PipelineId>,
    pub topic_schema: Arc<TopicSchema>,
    pub r#type: PipelineTriggerType,
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: Arc<PipelineExecutionLogMonitor>,
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

    fn find_pipeline_meta_service(&self) -> StdR<Arc<PipelineSchemaService>> {
        PipelineSchemaService::with(&self.principal.tenant_id)
    }

    fn load_pipelines(&self) -> StdR<Option<Vec<Arc<PipelineSchema>>>> {
        let pipeline_meta_service = self.find_pipeline_meta_service()?;
        let pipelines = match &self.pipeline_id {
            Some(pipeline_id) => {
                let pipeline = pipeline_meta_service.find_by_id(&pipeline_id)?;
                if let Some(pipeline) = pipeline {
                    let r#type = pipeline.r#type();
                    if *r#type.deref() != self.r#type {
                        return PipelineKernelErrorCode::TriggerTypeMismatchPipeline.msg(format!(
							"Defined pipeline[{}]'s trigger type[{}] does not match given trigger type[{}].",
							pipeline_id,
							r#type,
							self.r#type
						));
                    }
                    Some(vec![pipeline])
                } else {
                    return PipelineKernelErrorCode::TriggerPipelineNotFound
                        .msg(format!("Trigger pipeline[{}] not found.", &pipeline_id));
                }
            }
            _ => {
                let pipelines = pipeline_meta_service
                    .find_by_topic_and_pipeline_type(self.topic_schema.topic().topic_id.deref())?;
                if let Some(pipelines) = pipelines {
                    let pipelines: Vec<Arc<PipelineSchema>> = pipelines
                        .into_iter()
                        .filter(|p| *p.r#type().deref() == self.r#type)
                        .collect();
                    if pipelines.len() == 0 {
                        None
                    } else {
                        Some(pipelines)
                    }
                } else {
                    None
                }
            }
        };

        Ok(pipelines)
    }

    fn prepare_execution(
        &self,
        data: TopicData,
    ) -> StdR<(TopicDataId, Option<PipelineRunContext>)> {
        let topic_trigger = self.save_trigger_data(data)?;
        let topic_data_id = topic_trigger.internal_data_id.deref().clone();

        let pipelines = self.load_pipelines()?;
        if let Some(pipelines) = pipelines {
            let context = PipelineRunContext::new(self, topic_trigger, pipelines);
            Ok((topic_data_id, Some(context)))
        } else {
            println!(
                "No pipeline needs to be triggered by topic[id={}, name={}].",
                self.topic_schema.id(),
                self.topic_schema.name()
            );
            Ok((topic_data_id, None))
        }
    }

    pub fn execute(&self, data: TopicData) -> StdR<TopicDataId> {
        let (topic_data_id, context) = self.prepare_execution(data)?;
        if let Some(context) = context {
            PipelineRunner::execute(context)?;
        }
        Ok(topic_data_id)
    }

    pub async fn execute_async(&self, data: TopicData) -> StdR<TopicDataId> {
        let (topic_data_id, context) = self.prepare_execution(data)?;
        if let Some(context) = context {
            PipelineRunner::execute_async(context).await?;
        }
        Ok(topic_data_id)
    }
}
