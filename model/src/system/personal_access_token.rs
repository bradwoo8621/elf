use crate::{BaseDataModel, Storable, TenantId, UserBasedTuple, UserId};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

pub type PatId = String;

#[adapt_model(user_based)]
pub struct PersonalAccessToken {
    pub pat_id: Option<PatId>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub note: Option<String>,
    #[serde(with = "option_naive_datetime")]
    pub expired: Option<NaiveDateTime>,
    pub permissions: Option<Vec<String>>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
}
