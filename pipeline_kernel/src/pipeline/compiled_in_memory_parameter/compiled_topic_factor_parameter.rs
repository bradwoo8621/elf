use crate::{InMemoryParameter, PipelineExecutionVariables, PipelineKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    FactorType, StdErrorCode, StdR, TenantId, TopicDataProperty, TopicDataUtils, TopicDataValue,
};
use watchmen_runtime_model_kernel::{ArcTopicFactorParameter, TopicSchemaProvider, TopicService};

pub struct CompiledTopicFactorParameter {
    property: TopicDataProperty,
}

impl CompiledTopicFactorParameter {
    pub fn new(parameter: &Arc<ArcTopicFactorParameter>, tenant_id: &Arc<TenantId>) -> StdR<Self> {
        let topic_schema = TopicService::schema()?.by_id(parameter.topic_id.as_ref(), tenant_id)?;
        let property = match topic_schema.factor_by_id(parameter.factor_id.as_ref()) {
            None => {
                return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                    "Factor[{}] not found in topic[{}].",
                    &parameter.factor_id, &parameter.topic_id
                ));
            }
            Some(factor) => {
                let array = if let Some(index) = factor.name.find('.') {
                    // check if factor is hierarchical, get the top level is array or not
                    let first_name = &factor.name[0..index];
                    let first_factor = topic_schema.factor_by_name(&first_name.to_string());
                    if let Some(first_factor) = first_factor {
                        *first_factor.r#type.as_ref() == FactorType::Array
                    } else {
                        return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                            "Factor[{}] not found in topic[{}].",
                            &first_name, &parameter.topic_id
                        ));
                    }
                } else {
                    *factor.r#type.as_ref() == FactorType::Array
                };
                TopicDataProperty::of(&factor.name, array)
            }
        };

        Ok(CompiledTopicFactorParameter { property })
    }
}

/// topic factor parameter always retrieve data from current trigger data
impl InMemoryParameter for CompiledTopicFactorParameter {
    fn value_from<'a>(
        &self,
        variables: &'a PipelineExecutionVariables,
    ) -> StdR<&'a TopicDataValue> {
        variables.get_current_data()?.value_of(&self.property)
    }
}
