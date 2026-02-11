use crate::{
    ActionCompiler, CompiledAction, CompiledParameter, DataPath, DataPathSegment,
    PipelineKernelErrorCode,
};
use elf_base::{ErrorCode, StdR, StringUtils};
use elf_model::{TenantId, TopicId};
use elf_runtime_model_kernel::{
    ArcCopyToMemoryAction, ArcPipeline, ArcPipelineStage, ArcPipelineUnit,
    RuntimeModelKernelErrorCode, TopicSchema,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledCopyToMemoryAction {
    variable_path: DataPath,
    source: CompiledParameter,
}

impl ActionCompiler for CompiledCopyToMemoryAction {
    type SourceAction = ArcCopyToMemoryAction;

    fn compile(
        pipeline: &Arc<ArcPipeline>,
        stage: &Arc<ArcPipelineStage>,
        unit: &Arc<ArcPipelineUnit>,
        action: &ArcCopyToMemoryAction,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let variable_name = action.variable_name.as_str();
        if variable_name.is_blank() {
            return RuntimeModelKernelErrorCode::ActionVariableNameIsBlank.msg(format!(
                "Variable name of copy-to-memory action[action_id={}] cannot be blank.",
                action.action_id
            ));
        }
        let variable_path = DataPath::from_str(variable_name)?;
        for segment in variable_path.segments().iter() {
            match segment {
                DataPathSegment::Plain(_) => {}
                DataPathSegment::Func(_) => {
                    return PipelineKernelErrorCode::CopyToMemoryActionVariableIsNotPlain.msg(
                        format!(
                            "Variable name of copy-to-memory action[action_id={}] cannot contain function.",
                            action.action_id
                        ),
                    );
                }
            }
        }

        let source = CompiledParameter::compile(&action.source, topic_schemas, tenant_id)?;

        Ok(Self {
            variable_path,
            source,
        })
    }

    fn wrap_into_enum(compiled_action: Self) -> CompiledAction {
        CompiledAction::CopyToMemory(compiled_action)
    }
}
