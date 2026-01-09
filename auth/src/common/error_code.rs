use elf_base::ErrorCode;

pub enum AuthErrorCode {
    AuthenticationFailed,
    Unauthorized,
    Forbidden,
    // user related
    TenantIdMissedInUser,
    UserIdMissedInUser,
    NameMissedInUser,
    RoleMissedInUser,
}

impl ErrorCode for AuthErrorCode {
    fn code(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed => "AUTH-00001",
            Self::Unauthorized => "AUTH-00002",
            Self::Forbidden => "AUTH-00003",
            Self::TenantIdMissedInUser => "AUTH-00101",
            Self::UserIdMissedInUser => "AUTH-00102",
            Self::NameMissedInUser => "AUTH-00103",
            Self::RoleMissedInUser => "AUTH-00104",
        }
    }
}
