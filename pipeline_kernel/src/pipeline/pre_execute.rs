use crate::{
    PipelineExecuteInput, PipelineExecuteRequest, PipelineExecuteTopicData, PipelineKernelErrorCode,
};
use elf_base::{ErrorCode, StdR};
use elf_model::{PipelineId, PipelineTriggerType, TenantId, TopicData, TopicId};
use elf_runtime_model_kernel::{
    PipelineSchema, PipelineSchemaProvider, PipelineService, TopicSchema,
};
use std::ops::Deref;
use std::sync::Arc;

pub struct PipelinePreExecute;

impl PipelinePreExecute {
    fn save_trigger_data(
        topic_schema: &Arc<TopicSchema>,
        trigger_type: &Arc<PipelineTriggerType>,
        mut topic_data: TopicData,
    ) -> StdR<PipelineExecuteTopicData> {
        topic_schema.prepare(&mut topic_data)?;

        let topic = topic_schema.topic();

        if topic.is_synonym_topic() && trigger_type.is_insert() {
            PipelineExecuteTopicData::insert_into_synonym(topic_data)
        } else {
            match trigger_type.deref() {
                PipelineTriggerType::Insert => {
                    PipelineExecuteTopicData::insert(topic_data, topic_schema)
                }
                PipelineTriggerType::InsertOrMerge => {
                    PipelineExecuteTopicData::insert_or_merge(topic_data, topic_schema)
                }
                PipelineTriggerType::Merge => {
                    PipelineExecuteTopicData::merge(topic_data, topic_schema)
                }
                PipelineTriggerType::Delete => {
                    PipelineExecuteTopicData::delete(topic_data, topic_schema)
                }
            }
        }
    }

    fn load_pipeline_by_id(
        tenant_id: &TenantId,
        pipeline_id: &PipelineId,
        trigger_type: &PipelineTriggerType,
    ) -> StdR<Arc<PipelineSchema>> {
        let pipeline = PipelineService::schema()?.by_pipeline_id(pipeline_id, tenant_id)?;
        if let Some(pipeline) = pipeline {
            let r#type = pipeline.r#type();
            if !r#type.deref().matches(trigger_type) {
                return PipelineKernelErrorCode::TriggerTypeMismatchPipeline.msg(
                    format!(
                        "Defined pipeline[{}]'s trigger type[{}] does not match given trigger type[{}].",
                        pipeline_id,
                        r#type,
                        trigger_type
                    ));
            }
            Ok(pipeline)
        } else {
            PipelineKernelErrorCode::TriggerPipelineNotFound
                .msg(format!("Trigger pipeline[{}] not found.", &pipeline_id))
        }
    }

    fn load_pipelines_by_topic_id(
        tenant_id: &TenantId,
        topic_id: &TopicId,
        trigger_type: &PipelineTriggerType,
    ) -> StdR<Option<Vec<Arc<PipelineSchema>>>> {
        let pipelines = PipelineService::schema()?.by_topic_id(topic_id, tenant_id)?;
        if let Some(pipelines) = pipelines {
            let pipelines: Vec<Arc<PipelineSchema>> = pipelines
                .into_iter()
                .filter(|p| p.r#type().matches(trigger_type))
                .collect();
            if pipelines.len() == 0 {
                Ok(None)
            } else {
                Ok(Some(pipelines))
            }
        } else {
            Ok(None)
        }
    }

    /// - save topic data
    /// - find pipeline(s)
    /// - build execute request
    pub fn pre_execute(
        input: PipelineExecuteInput,
        pipeline_id: Option<&PipelineId>,
    ) -> StdR<PipelineExecuteRequest> {
        let topic_schema = input.topic_schema();
        let trigger_type = input.trigger_type();
        let principal = input.principal();
        let trace_id = input.trace_id();

        let topic_data = input.topic_data();
        let execute_topic_data = Self::save_trigger_data(&topic_schema, &trigger_type, topic_data)?;

        let tenant_id = principal.tenant_id();
        let pipeline_schemas = match pipeline_id {
            Some(pipeline_id) => {
                let pipeline_schema =
                    Self::load_pipeline_by_id(tenant_id, pipeline_id, &trigger_type)?;
                vec![pipeline_schema]
            }
            None => {
                if let Some(pipeline_schemas) = Self::load_pipelines_by_topic_id(
                    tenant_id,
                    topic_schema.tenant_id(),
                    &trigger_type,
                )? {
                    pipeline_schemas
                } else {
                    vec![]
                }
            }
        };

        Ok(PipelineExecuteRequest::create(
            principal,
            execute_topic_data,
            topic_schema,
            pipeline_schemas,
            trace_id,
        ))
    }
}
