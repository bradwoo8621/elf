use crate::{ArcHelper, ArcParameter};
use elf_base::StdR;
use elf_model::{AggregateArithmetic, FactorId, MappingFactor};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcMappingFactor {
    pub source: Arc<ArcParameter>,
    pub factor_id: Arc<FactorId>,
    pub arithmetic: Arc<AggregateArithmetic>,
}

impl ArcHelper for ArcMappingFactor {}

impl ArcMappingFactor {
    pub fn new(mapping: MappingFactor) -> StdR<Arc<Self>> {
        let factor_id = Self::factor_id(mapping.factor_id, || "Mapping factor")?;
        let source = Self::action_source(mapping.source, || {
            format!("Mapping factor to[{}]", factor_id)
        })?;

        Ok(Arc::new(Self {
            source,
            factor_id,
            arithmetic: Arc::new(mapping.arithmetic.unwrap_or(AggregateArithmetic::None)),
        }))
    }
}
