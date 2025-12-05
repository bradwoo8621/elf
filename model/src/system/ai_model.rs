use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, StdErrCode, StdErrorCode, StdR, Storable,
    TenantBasedTuple, TenantId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
#[pattern = "keep-same"]
pub enum AiModelProvider {
    Anthropic,
    Microsoft,
    OpenAI,
    AWS,
    #[display = "Hugging Face"]
    HuggingFace,
    Google,
    Ollama,
}

pub type AIModelId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct AIModel {
    pub model_id: Option<AIModelId>,
    pub enable_monitor: Option<bool>,
    pub llm_provider: Option<AiModelProvider>,
    pub base_url: Option<String>,
    pub model_name: Option<String>,
    pub model_version: Option<String>,
    pub model_token: Option<String>,
    pub embedding_provider: Option<AiModelProvider>,
    pub base_embedding_url: Option<String>,
    pub embedding_name: Option<String>,
    pub embedding_version: Option<String>,
    pub embedding_token: Option<String>,
}
