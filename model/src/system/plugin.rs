use crate::{
    Auditable, BaseDataModel, ModelErrorCode, OptimisticLock, Storable, TenantBasedTuple, TenantId,
    Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum PluginType {
    Streamlit,
    Jupyter,
}

#[derive(Display, Serde, StrEnum)]
pub enum PluginApplyTo {
    Achievement,
}

pub type PluginId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Plugin {
    pub plugin_id: Option<PluginId>,
    pub plugin_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<PluginType>,
    pub apply_to: Option<PluginApplyTo>,
    /// value is parameter name
    pub params: Option<Vec<String>>,
    /// value is result name
    pub results: Option<Vec<String>>,
}
