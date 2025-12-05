use watchmen_model::{StdErrCode, StdErrorCode, StdR};
use watchmen_model_marco::{Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum AuthenticationScheme {
    #[display = "Bearer"]
    Bearer,
    Pat,
    #[display = "external"]
    EXT,
}

pub struct AuthenticationDetails {
    scheme: AuthenticationScheme,
    token: String,
}

impl AuthenticationDetails {
    pub fn new(scheme: AuthenticationScheme, token: String) -> Self {
        Self { scheme, token }
    }

    pub fn scheme(&self) -> &AuthenticationScheme {
        &self.scheme
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}
