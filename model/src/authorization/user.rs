use crate::serde::option_naive_datetime;
use crate::{
    serde_for_enum, Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId,
    Tuple, UserGroupId, UserId,
};
use std::fmt;
use watchmen_model_marco::adapt_model;

pub enum UserRole {
    Console,
    Admin,
    SuperAdmin,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Console => write!(f, "console"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::SuperAdmin => write!(f, "superadmin"),
        }
    }
}

serde_for_enum! {
    UserRole {
        Console => "console",
        Admin => "admin",
        SuperAdmin => "superadmin"
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
