use crate::{Auditable, BaseDataModel, Storable, TenantBasedTuple, TenantId, Tuple, UserId};
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type PackageVersionId = String;

#[adapt_model(tenant_based)]
pub struct PackageVersion {
    pub version_id: Option<PackageVersionId>,
    pub pre_version: Option<String>,
    pub curr_version: Option<String>,
}
