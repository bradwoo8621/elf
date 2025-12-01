use crate::{
    ArcHelper, ArcParameterJoint, ArcPipelineStage, RuntimeModelKernelErrorCode,
};
use std::sync::Arc;
use watchmen_model::{
    Pipeline, PipelineId, PipelineTriggerType, StdErrorCode, StdR, TenantId, TopicId,
};

#[derive(Debug)]
pub struct ArcPipeline {
    pub pipeline_id: Arc<PipelineId>,
    pub topic_id: Arc<TopicId>,
    pub name: Arc<String>,
    pub r#type: Arc<PipelineTriggerType>,
    pub stages: Arc<Vec<Arc<ArcPipelineStage>>>,
    pub enabled: bool,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
    pub tenant_id: Arc<TenantId>,
    pub version: u32,
}

impl ArcHelper for ArcPipeline {}

impl ArcPipeline {
    pub fn new(pipeline: Pipeline) -> StdR<Arc<Self>> {
        let pipeline_id = Self::not_blank(
            pipeline.pipeline_id,
            || RuntimeModelKernelErrorCode::PipelineIdMissed.msg("Pipeline must have an id."),
            || RuntimeModelKernelErrorCode::PipelineIdIsBlank.msg("Pipeline id cannot be blank."),
        )?;
        let name = Self::name(pipeline.name, || format!("Pipeline[{}]", pipeline_id))?;
        let tenant_id =
            Self::tenant_id(pipeline.tenant_id, || format!("Pipeline[{}]", pipeline_id))?;
        let topic_id = Self::topic_id(pipeline.topic_id, || format!("Pipeline[{}]", name))?;
        let r#type = Self::must(pipeline.r#type, || {
            RuntimeModelKernelErrorCode::PipelineTypeMissed
                .msg(format!("Pipeline[{}] must have a type.", pipeline_id))
        })?;
        let arc_stages = Self::must_vec(pipeline.stages, ArcPipelineStage::new, || {
            RuntimeModelKernelErrorCode::PipelineStageMissed
                .msg(format!("Pipeline[{}] must have stage.", pipeline_id))
        })?;
        let on = Self::conditional(pipeline.conditional, pipeline.on, || {
            format!(
                "Pipeline[{}] must have condition when conditional is true.",
                name
            )
        })?;

        Ok(Arc::new(Self {
            pipeline_id,
            topic_id,
            name,
            r#type,
            stages: arc_stages,
            enabled: pipeline.enabled.unwrap_or(true),
            conditional: on.is_some(),
            on,
            tenant_id,
            version: pipeline.version.unwrap_or(0),
        }))
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_conditional(&self) -> bool {
        self.conditional
    }
}
