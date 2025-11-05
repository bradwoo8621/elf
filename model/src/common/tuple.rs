use super::Auditable;
use super::TenantId;
use super::UserId;

pub trait Tuple: Auditable {}

pub trait TenantBasedTuple: Tuple {
    fn tenant_id(&self) -> Option<TenantId>;
}

pub trait UserBasedTuple: TenantBasedTuple {
    fn user_id(&self) -> Option<UserId>;
}
