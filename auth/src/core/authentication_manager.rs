use crate::{AuthErrorCode, AuthenticationDetails, AuthenticationProvider, AuthenticationScheme};
use watchmen_model::{StdErr, StdErrorCode, User};

pub struct AuthenticationManager {
    pub providers: Vec<Box<dyn AuthenticationProvider>>,
}

impl AuthenticationManager {
    pub fn register_provider(
        mut self,
        provider: Box<dyn AuthenticationProvider>,
    ) -> AuthenticationManager {
        self.providers.push(provider);
        return self;
    }

    pub fn authenticate_details(&self, details: &AuthenticationDetails) -> Result<User, StdErr> {
        for provider in &self.providers {
            if provider.accept(details) {
                if let Some(user) = provider.authenticate(details) {
                    return Ok(user);
                }
            }
        }
        StdErr::of(
            AuthErrorCode::AuthenticationFailed.code(),
            "Authentication failed",
        )
    }

    pub fn authenticate(
        &self,
        scheme: AuthenticationScheme,
        token: String,
    ) -> Result<User, StdErr> {
        self.authenticate_details(&AuthenticationDetails::new(scheme, token))
    }
}
