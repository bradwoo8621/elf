use watchmen_base::StdR;
use watchmen_model::{KeyStoreParams, TenantId};

/// TODO keystore meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out keystore meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
///
/// Some key might be time-related, it is new feature
pub struct KeyStoreMetaService;

impl KeyStoreMetaService {
    pub fn find(_key_type: &String, _tenant_id: &TenantId) -> StdR<Option<KeyStoreParams>> {
        todo!("implement find for KeyStoreMetaService")
    }

    pub fn find_by_key(
        _key_type: &String,
        _key_key: &String,
        _tenant_id: &TenantId,
    ) -> StdR<Option<KeyStoreParams>> {
        todo!("implement find_by_key for KeyStoreMetaService")
    }
}
