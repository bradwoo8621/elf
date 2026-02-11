use crate::{CompiledAction, DataPath, DataPathSegment, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR, StringUtils};
use elf_model::{FactorId, PipelineActionId, PipelineActionType, TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcFactor, ArcPipeline, ArcPipelineStage, ArcPipelineUnit, RuntimeModelKernelErrorCode,
    TopicSchema, TopicSchemaProvider, TopicService,
};
use std::collections::HashMap;
use std::sync::Arc;

pub trait ActionCompiler
where
    Self: Sized,
{
    type SourceAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &Self::SourceAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self>;

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction;
}

pub struct ActionCompilerHelper;

impl ActionCompilerHelper {
    pub fn get_variable_name(
        variable_name: &str,
        action_id: &PipelineActionId,
        action_type: &PipelineActionType,
    ) -> StdR<DataPath> {
        if variable_name.is_blank() {
            return RuntimeModelKernelErrorCode::ActionVariableNameIsBlank.msg(format!(
                "Variable name of {} action[action_id={}] cannot be blank.",
                action_type, action_id
            ));
        }
        let variable_path = DataPath::from_str(variable_name)?;
        for segment in variable_path.segments().iter() {
            match segment {
                DataPathSegment::Plain(_) => {}
                DataPathSegment::Func(_) => {
                    return PipelineKernelErrorCode::CopyToMemoryActionVariableIsNotPlain.msg(
                        format!(
                            "Variable name of {} action[action_id={}] cannot contain function.",
                            action_type, action_id
                        ),
                    );
                }
            }
        }

        Ok(variable_path)
    }

    pub fn find_topic_schema(
        topic_id: &Arc<TopicId>,
        tenant_id: &Arc<TenantId>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
    ) -> StdR<Arc<TopicSchema>> {
        if let Some(topic_schema) = topic_schemas.get(topic_id) {
            Ok(topic_schema.clone())
        } else {
            let topic_schema = TopicService::schema()?.by_id(topic_id, tenant_id)?;
            topic_schemas.insert(topic_id.clone(), topic_schema.clone());
            Ok(topic_schema)
        }
    }

    pub fn find_factor(topic_schema: &TopicSchema, factor_id: &FactorId) -> StdR<Arc<ArcFactor>> {
        if let Some(factor) = topic_schema.factor_by_id(factor_id) {
            Ok(factor.clone())
        } else {
            return RuntimeModelKernelErrorCode::TopicFactorMissed.msg(format!(
                "Factor[factor_id={}] not found in topic[topic_id={}].",
                factor_id,
                topic_schema.topic_id()
            ));
        }
    }
}
