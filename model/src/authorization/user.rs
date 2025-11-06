use crate::serde::option_naive_datetime;
use crate::{
    serde_for_enum, Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId,
    Tuple, UserGroupId, UserId,
};
use watchmen_model_marco::{adapt_model, Display};

#[derive(Display)]
pub enum UserRole {
    Console,
    Admin,
    Superadmin,
}

serde_for_enum! {
    UserRole {
        Console => "console",
        Admin => "admin",
        Superadmin => "superadmin"
    }
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
