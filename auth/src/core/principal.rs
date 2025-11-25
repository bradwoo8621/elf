use crate::{AuthErrorCode, AuthenticationScheme, Authorization};
use watchmen_model::{StdErr, StdErrorCode, TenantId, User, UserId, UserRole};

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

    pub fn from_user(user: User) -> Result<Principal, StdErr> {
        if user.tenant_id.is_none() {
            return StdErr::of(
                AuthErrorCode::TenantIdMissedInUser.code(),
                "Tenant id is missing in user.",
            );
        }
        if user.user_id.is_none() {
            return StdErr::of(
                AuthErrorCode::UserIdMissedInUser.code(),
                "User id is missing in user.",
            );
        }
        if user.name.is_none() {
            return StdErr::of(
                AuthErrorCode::NameMissedInUser.code(),
                "Name is missing in user.",
            );
        }
        if user.role.is_none() {
            return StdErr::of(
                AuthErrorCode::RoleMissedInUser.code(),
                "Role is missing in user.",
            );
        }

        Ok(Principal {
            tenant_id: user.tenant_id.unwrap(),
            user_id: user.user_id.unwrap(),
            name: user.name.unwrap(),
            role: user.role.unwrap(),
        })
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

    pub fn authorize_token(
        authorization: &Authorization,
        scheme: AuthenticationScheme,
        token: String,
    ) -> Result<Self, StdErr> {
        let user = authorization.authorize_token(scheme, token)?;
        Principal::from_user(user)
    }
}
