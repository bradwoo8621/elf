use crate::{
    Auditable, BaseDataModel, ModelErrorCode, PluginId, Storable, TenantId, UserBasedTuple, UserId,
};
use elf_base::serde::option_naive_datetime;
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum AchievementPluginTaskStatus {
    Submitted,
    Sent,
    Success,
    Failed,
}

pub type AchievementPluginTaskId = String;

#[adapt_model(audit, user_based)]
pub struct AchievementPluginTask {
    pub achievement_task_id: Option<AchievementPluginTaskId>,
    /// TODO refactor: objective achievement is dropped
    /// achievementId: AchievementId = None
    pub achievement_id: Option<String>,
    pub plugin_id: Option<PluginId>,
    pub status: Option<AchievementPluginTaskStatus>,
    pub url: Option<String>,
}
