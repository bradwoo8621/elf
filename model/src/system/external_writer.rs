use crate::{
    Auditable, BaseDataModel, ModelErrorCode, OptimisticLock, Storable, TenantBasedTuple, TenantId,
    Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum ExternalWriterType {
    StandardWriter,
    ElasticSearchWriter,
}

pub type ExternalWriterId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct ExternalWriter {
    pub writer_id: Option<ExternalWriterId>,
    pub writer_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<ExternalWriterType>,
    /// personal access token
    pub pat: Option<String>,
    pub url: Option<String>,
}
