use crate::{BaseDataModel, Storable, TenantId, UserRole};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum TokenType {
    Bearer,
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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenRecitation {
    Std(Token),
    Saml(SamlToken),
    Oidc(OidcToken),
}
