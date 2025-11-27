use crate::{AuthErrorCode, AuthenticationManager, AuthenticationScheme};
use watchmen_model::{StdErrorCode, StdR, User, UserRole};

pub struct Authorization {
    // TODO where to get the authenticator?
    authenticator: AuthenticationManager,
    /// allowed roles
    roles: Vec<UserRole>,
}

impl Authorization {
    pub fn new(authenticator: AuthenticationManager, roles: Vec<UserRole>) -> Self {
        Authorization {
            authenticator,
            roles,
        }
    }

    /// check the user role against allowed roles
    pub fn authorize(&self, user: Option<User>) -> StdR<User> {
        match user {
            Some(u) => {
                if let Some(role) = &u.role {
                    if self.roles.contains(role) {
                        Ok(u)
                    } else {
                        // given user role is not allowed
                        AuthErrorCode::Forbidden.msg("Forbidden")
                    }
                } else {
                    // user has no role assigned
                    AuthErrorCode::Unauthorized.msg("Unauthorized")
                }
            }
            // no user authenticated
            None => AuthErrorCode::Unauthorized.msg("Unauthorized"),
        }
    }

    pub fn authorize_token(&self, scheme: AuthenticationScheme, token: String) -> StdR<User> {
        if let Ok(user) = self.authenticator.authenticate(scheme, token) {
            self.authorize(Some(user))
        } else {
            AuthErrorCode::Unauthorized.msg("Unauthorized")
        }
    }
}
