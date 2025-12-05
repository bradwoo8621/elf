use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, StdErrCode, StdErrorCode, StdR, Storable,
    TenantBasedTuple, TenantId, Tuple, UserGroupId,
};
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum, Clone)]
pub enum UserRole {
    Console,
    Admin,
    #[display = "superadmin"]
    SuperAdmin,
}

impl PartialEq<UserRole> for UserRole {
    fn eq(&self, other: &UserRole) -> bool {
        match (self, other) {
            (UserRole::Console, UserRole::Console) => true,
            (UserRole::Admin, UserRole::Admin) => true,
            (UserRole::SuperAdmin, UserRole::SuperAdmin) => true,
            _ => false,
        }
    }
}

pub type UserId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct User {
    pub user_id: Option<UserId>,
    pub name: Option<String>,
    pub nick_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub group_ids: Option<Vec<UserGroupId>>,
    pub role: Option<UserRole>,
}
