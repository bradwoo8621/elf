use crate::{AuthErrorCode, AuthenticationManager, AuthenticationScheme};
use watchmen_model::{StdErr, StdErrorCode, User, UserRole};

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
    pub fn authorize(&self, user: Option<User>) -> Result<User, StdErr> {
        match user {
            Some(u) => {
                if let Some(role) = &u.role {
                    if self.roles.contains(role) {
                        Ok(u)
                    } else {
                        // given user role is not allowed
                        StdErr::of(AuthErrorCode::Forbidden.code(), "Forbidden")
                    }
                } else {
                    // user has no role assigned
                    StdErr::of(AuthErrorCode::Unauthorized.code(), "Unauthorized")
                }
            }
            // no user authenticated
            None => StdErr::of(AuthErrorCode::Unauthorized.code(), "Unauthorized"),
        }
    }

    pub fn authorize_token(
        &self,
        scheme: AuthenticationScheme,
        token: String,
    ) -> Result<User, StdErr> {
        if let Ok(user) = self.authenticator.authenticate(scheme, token) {
            self.authorize(Some(user))
        } else {
            StdErr::of(AuthErrorCode::Unauthorized.code(), "Unauthorized")
        }
    }
}
