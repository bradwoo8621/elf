use crate::{BaseDataModel, StdErrCode, StdErrorCode, StdR, Storable, TenantId, UserRole};
use serde::Deserialize;
use watchmen_model_marco::{adapt_model, Display, Serde, StrEnum, VariousStructTypes};

#[derive(Display, Serde, StrEnum)]
pub enum TokenType {
    Bearer,
}

impl TokenType {
    pub fn value(self) -> &'static str {
        match self {
            TokenType::Bearer => "Bearer",
        }
    }
}

#[adapt_model(storable)]
pub struct Token {
    pub access_token: Option<String>,
    pub token_type: Option<TokenType>,
    pub role: Option<UserRole>,
    pub tenant_id: Option<TenantId>,
}

#[adapt_model(storable)]
pub struct SamlToken {
    pub account_ame: Option<String>,
    /// [Token]
    pub access_token: Option<String>,
    pub token_type: Option<TokenType>,
    pub role: Option<UserRole>,
    pub tenant_id: Option<TenantId>,
}

#[adapt_model(storable)]
pub struct OidcToken {
    pub account_ame: Option<String>,
    /// [Token]
    pub access_token: Option<String>,
    pub token_type: Option<TokenType>,
    pub role: Option<UserRole>,
    pub tenant_id: Option<TenantId>,
}

#[derive(Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum TokenRecitation {
    Std(Token),
    Saml(SamlToken),
    Oidc(OidcToken),
}
