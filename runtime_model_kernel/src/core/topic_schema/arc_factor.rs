use crate::RuntimeModelKernelErrorCode;
use std::sync::Arc;
use watchmen_model::{
    EnumId, Factor, FactorEncryptMethod, FactorId, FactorType, StdErrorCode, StdR,
};

/// An Arc-wrapped Factor structure with optional fields.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
#[derive(Debug)]
pub struct ArcFactor {
    pub factor_id: Option<Arc<FactorId>>,
    pub r#type: Arc<FactorType>,
    pub name: Arc<String>,
    pub enum_id: Option<Arc<EnumId>>,
    pub label: Option<Arc<String>>,
    pub default_value: Option<Arc<String>>,
    pub flatten: Option<bool>,
    pub encrypt: Option<Arc<FactorEncryptMethod>>,
}

impl ArcFactor {
    pub fn new(factor: Factor) -> StdR<Arc<Self>> {
        if factor.name.is_none() {
            return RuntimeModelKernelErrorCode::FactorNameMissed.msg("Factor must have a name.");
        }
        let name = Arc::new(factor.name.unwrap());
        if factor.r#type.is_none() {
            return RuntimeModelKernelErrorCode::FactorTypeMissed.msg("Factor must have a name.");
        }

        Ok(Arc::new(Self {
            factor_id: factor.factor_id.map(Arc::new),
            r#type: Arc::new(factor.r#type.unwrap()),
            name,
            enum_id: factor.enum_id.map(Arc::new),
            label: factor.label.map(Arc::new),
            default_value: factor.default_value.map(Arc::new),
            flatten: factor.flatten,
            encrypt: factor.encrypt.map(Arc::new),
        }))
    }

    pub fn is_flatten(&self) -> bool {
        self.flatten.unwrap_or_else(|| false)
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    pub fn is_date_or_time(&self) -> bool {
        self.r#type.is_date_or_time()
    }
}
