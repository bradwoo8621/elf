use crate::{
	Auditable, BaseDataModel, ModelErrorCode, OptimisticLock, Storable, TenantBasedTuple, TenantId,
	Tuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[adapt_model(storable)]
pub struct NotificationParam {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Display, Serde, StrEnum)]
pub enum NotificationType {
    Email,
    #[display = "url"]
    WebUrl,
    Slack,
    Feishu,
}

pub type NotificationDefinitionId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct NotificationDefinition {
    pub notification_id: Option<NotificationDefinitionId>,
    pub r#type: Option<NotificationType>,
    pub params: Option<Vec<NotificationParam>>,
    pub user_id: Option<UserId>,
}
