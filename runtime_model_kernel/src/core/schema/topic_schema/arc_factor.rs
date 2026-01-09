use crate::{ArcHelper, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{EnumId, Factor, FactorEncryptMethod, FactorId, FactorType};
use std::sync::Arc;

/// An Arc-wrapped Factor structure with optional fields.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
#[derive(Debug)]
pub struct ArcFactor {
    pub factor_id: Arc<FactorId>,
    pub r#type: Arc<FactorType>,
    pub name: Arc<String>,
    pub enum_id: Option<Arc<EnumId>>,
    pub label: Option<Arc<String>>,
    pub default_value: Option<Arc<String>>,
    pub flatten: bool,
    pub encrypt: Option<Arc<FactorEncryptMethod>>,
}

impl ArcHelper for ArcFactor {}

impl ArcFactor {
    pub fn new(factor: Factor) -> StdR<Arc<Self>> {
        let factor_id = Self::factor_id(factor.factor_id, || "Factor")?;
        let name = Self::name(factor.name, || format!("Factor[{}]", factor_id))?;
        let r#type = Self::must(factor.r#type, || {
            RuntimeModelKernelErrorCode::FactorTypeMissed
                .msg(format!("Factor[{}] must have a type.", factor_id))
        })?;

        Ok(Arc::new(Self {
            factor_id,
            r#type,
            name,
            enum_id: Self::arc(factor.enum_id),
            label: Self::arc(factor.label),
            default_value: Self::arc(factor.default_value),
            flatten: factor.flatten.unwrap_or(false),
            encrypt: Self::arc(factor.encrypt),
        }))
    }

    pub fn is_flatten(&self) -> bool {
        self.flatten
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    pub fn is_date_or_time(&self) -> bool {
        self.r#type.is_date_or_time()
    }
}
