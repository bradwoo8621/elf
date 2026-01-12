use elf_base::{StdR, VoidR};
use elf_model::{KeyStoreParams, TenantId};

/// TODO keystore meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out keystore meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
///
/// Some key might be time-related, it is new feature
pub struct KeyStoreService;

impl KeyStoreService {
    /// TODO implement create for KeyStoreMetaService
    pub fn create(
        _key_type: &String,
        _key_key: &Option<String>,
        _tenant_id: &TenantId,
        _params: KeyStoreParams,
    ) -> VoidR {
        Ok(())
    }

    /// TODO implement find for KeyStoreMetaService
    pub fn find(
        _key_type: &String,
        _key_key: &Option<String>,
        _tenant_id: &TenantId,
    ) -> StdR<Option<KeyStoreParams>> {
        Ok(None)
    }
}
