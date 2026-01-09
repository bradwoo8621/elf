use crate::{
    Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId, TopicId, Tuple,
    UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type CatalogId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Catalog {
    pub catalog_id: Option<CatalogId>,
    pub name: Option<String>,
    pub topic_ids: Option<Vec<TopicId>>,
    pub tech_owner_id: Option<UserId>,
    pub biz_owner_id: Option<UserId>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
}

#[adapt_model(storable)]
pub struct CatalogCriteria {
    pub name: Option<String>,
    pub topic_id: Option<TopicId>,
    pub tech_owner_id: Option<UserId>,
    pub biz_owner_id: Option<UserId>,
}
