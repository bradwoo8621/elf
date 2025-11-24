use watchmen_model::{TenantId, UserId, UserRole};

pub struct Principal {
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub name: String,
    pub role: UserRole,
}

impl Principal {
    pub fn is_admin(&self) -> bool {
        match self.role {
            UserRole::Admin | UserRole::SuperAdmin => true,
            _ => false,
        }
    }

    pub fn is_tenant_admin(&self) -> bool {
        match self.role {
            UserRole::Admin => true,
            _ => false,
        }
    }

    pub fn is_super_admin(&self) -> bool {
        match self.role {
            UserRole::SuperAdmin => true,
            _ => false,
        }
    }

    /// - [tenant_id]: -1,
    /// - [user_id]: 1,
    /// - [user_name]: imma-super
    /// - [role]: always be [UserRole::SuperAdmin]
    pub fn fake_super_admin() -> Principal {
        Principal {
            tenant_id: String::from("-1"),
            user_id: String::from("1"),
            name: String::from("imma-super"),
            role: UserRole::SuperAdmin,
        }
    }

    /// use default values if parameters are None
    /// - [tenant_id]: -1,
    /// - [user_id]: 1,
    /// - [user_name]: imma-super
    /// - [role]: always be [UserRole::Admin]
    pub fn fake_tenant_admin(
        tenant_id: Option<TenantId>,
        user_id: Option<UserId>,
        user_name: Option<String>,
    ) -> Principal {
        Principal {
            tenant_id: tenant_id.unwrap_or(String::from("-1")),
            user_id: user_id.unwrap_or(String::from("1")),
            name: user_name.unwrap_or(String::from("imma-super")),
            role: UserRole::Admin,
        }
    }
}
