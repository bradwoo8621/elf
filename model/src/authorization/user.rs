use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple,
    UserGroupId, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum UserRole {
    Console,
    Admin,
    #[display = "superadmin"]
    SuperAdmin,
}

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
