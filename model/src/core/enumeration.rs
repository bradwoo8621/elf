use crate::{
    Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type EnumItemId = String;

#[adapt_model(storable)]
pub struct EnumItem {
    pub item_id: Option<EnumItemId>,
    pub code: Option<String>,
    pub label: Option<String>,
    pub parent_code: Option<String>,
    pub replace_code: Option<String>,
    pub enum_id: Option<EnumId>,
    pub tenant_id: Option<TenantId>,
}

pub type EnumId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Enum {
    pub enum_id: Option<EnumId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_enum_id: Option<EnumId>,
    pub items: Option<Vec<EnumItem>>,
}
