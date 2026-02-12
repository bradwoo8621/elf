use crate::{ActionCompilerHelper, CompiledParameter};
use elf_base::StdR;
use elf_model::{AggregateArithmetic, FactorId, TenantId, TopicId};
use elf_runtime_model_kernel::{ArcFactor, ArcMappingFactor, ArcParameter, TopicSchema};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CompiledMappingFactor {
    source_parameter: CompiledParameter,
    target_factor: Arc<ArcFactor>,
    aggregate_arithmetic: AggregateArithmetic,
}

impl CompiledMappingFactor {
    pub fn one(
        target_topic_schema: &Arc<TopicSchema>,
        source: &Arc<ArcParameter>,
        factor_id: &FactorId,
        aggregate_arithmetic: &Arc<AggregateArithmetic>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Self> {
        let source_parameter = CompiledParameter::compile(source, topic_schemas, tenant_id)?;
        let target_factor = ActionCompilerHelper::find_factor(target_topic_schema, factor_id)?;
        let aggregate_arithmetic =
            ActionCompilerHelper::unwrap_aggregate_arithmetic(aggregate_arithmetic);

        Ok(Self {
            source_parameter,
            target_factor,
            aggregate_arithmetic,
        })
    }

    pub fn create(
        target_topic_schema: &Arc<TopicSchema>,
        definition: &Vec<Arc<ArcMappingFactor>>,
        topic_schemas: &mut HashMap<Arc<TopicId>, Arc<TopicSchema>>,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Vec<Self>> {
        let mut compiled = vec![];

        for item in definition.iter() {
            compiled.push(Self::one(
                target_topic_schema,
                &item.source,
                &item.factor_id,
                &item.arithmetic,
                topic_schemas,
                tenant_id,
            )?);
        }

        Ok(compiled)
    }
}
