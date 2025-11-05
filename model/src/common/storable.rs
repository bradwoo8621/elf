use super::BaseDataModel;
use super::UserId;
use chrono::NaiveDateTime;

pub trait Storable: BaseDataModel {}

pub trait Auditable: Storable {
    fn created_at(&self) -> Option<NaiveDateTime>;
    fn created_by(&self) -> Option<UserId>;
    fn last_modified_at(&self) -> Option<NaiveDateTime>;
    fn last_modified_by(&self) -> Option<UserId>;
}

pub trait OptimisticLock: Storable {
    fn version(&self) -> Option<u32>;
}

pub trait LastVisit: Storable {
    fn last_visit_time(&self) -> Option<NaiveDateTime>;
}
