use watchmen_model::StdErrorCode;

pub enum AuthErrorCode {
    AuthenticationFailed,
    Unauthorized,
    Forbidden,
}

impl StdErrorCode for AuthErrorCode {
    fn code(&self) -> &'static str {
        match self {
            AuthErrorCode::AuthenticationFailed => "AUTH-00001",
            AuthErrorCode::Unauthorized => "AUTH-00002",
            AuthErrorCode::Forbidden => "AUTH-00003",
        }
    }
}
