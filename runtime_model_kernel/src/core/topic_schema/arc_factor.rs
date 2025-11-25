use std::sync::Arc;
use watchmen_model::{EnumId, Factor, FactorEncryptMethod, FactorId, FactorIndexGroup, FactorType};

/// An Arc-wrapped Factor structure with optional fields.
/// This struct uses Arc pointers for each field to allow efficient sharing
/// of data across multiple threads without unnecessary cloning.
pub struct ArcFactor {
    pub factor_id: Option<Arc<FactorId>>,
    pub r#type: Option<Arc<FactorType>>,
    pub name: Option<Arc<String>>,
    pub enum_id: Option<Arc<EnumId>>,
    pub label: Option<Arc<String>>,
    pub description: Option<Arc<String>>,
    pub default_value: Option<Arc<String>>,
    pub flatten: Option<bool>,
    pub index_group: Option<Arc<FactorIndexGroup>>,
    pub encrypt: Option<Arc<FactorEncryptMethod>>,
    pub precision: Option<Arc<String>>,
}

impl ArcFactor {
    pub fn from(factor: Factor) -> Arc<ArcFactor> {
        Arc::new(ArcFactor {
            factor_id: factor.factor_id.map(Arc::new),
            r#type: factor.r#type.map(Arc::new),
            name: factor.name.map(Arc::new),
            enum_id: factor.enum_id.map(Arc::new),
            label: factor.label.map(Arc::new),
            description: factor.description.map(Arc::new),
            default_value: factor.default_value.map(Arc::new),
            flatten: factor.flatten,
            index_group: factor.index_group.map(Arc::new),
            encrypt: factor.encrypt.map(Arc::new),
            precision: factor.precision.map(Arc::new),
        })
    }

    pub fn is_flatten(&self) -> bool {
        self.flatten.unwrap_or_else(|| false)
    }

    pub fn is_date_or_time(&self) -> bool {
        match &self.r#type {
            Some(t) => match t.as_ref() {
                FactorType::Date
                | FactorType::Time
                | FactorType::Datetime
                | FactorType::FullDatetime
                | FactorType::DateOfBirth => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }
}
