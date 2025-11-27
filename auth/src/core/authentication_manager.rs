use crate::{AuthErrorCode, AuthenticationDetails, AuthenticationProvider, AuthenticationScheme};
use watchmen_model::{StdErrorCode, StdR, User};

pub struct AuthenticationManager {
    providers: Vec<Box<dyn AuthenticationProvider>>,
}

impl AuthenticationManager {
    pub fn new() -> AuthenticationManager {
        AuthenticationManager {
            providers: Vec::new(),
        }
    }

    pub fn of(providers: Vec<Box<dyn AuthenticationProvider>>) -> AuthenticationManager {
        AuthenticationManager { providers }
    }

    pub fn register_provider(
        mut self,
        provider: Box<dyn AuthenticationProvider>,
    ) -> AuthenticationManager {
        self.providers.push(provider);
        return self;
    }

    pub fn authenticate_details(&self, details: &AuthenticationDetails) -> StdR<User> {
        for provider in &self.providers {
            if provider.accept(details) {
                if let Some(user) = provider.authenticate(details) {
                    return Ok(user);
                }
            }
        }
        AuthErrorCode::AuthenticationFailed.msg("Authentication failed")
    }

    pub fn authenticate(&self, scheme: AuthenticationScheme, token: String) -> StdR<User> {
        self.authenticate_details(&AuthenticationDetails::new(scheme, token))
    }
}
