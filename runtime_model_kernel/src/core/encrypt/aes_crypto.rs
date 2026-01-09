use crate::{Crypto, CryptoUtils, KeyStoreService, RuntimeModelKernelErrorCode};
use aes::Aes256;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use cfb_mode::{
    cipher::{AsyncStreamCipher, KeyIvInit}, Decryptor as CfbDecryptor,
    Encryptor as CfbEncryptor,
};
use elf_base::{EnvConfig, ErrorCode, StdErrCode, StdR, VoidR};
use elf_model::{FactorEncryptMethod, KeyStoreValue, TenantId, TopicDataValue};
use std::collections::HashMap;
use std::iter::repeat;
use std::ops::Deref;
use std::sync::{Arc, OnceLock, RwLock};
use subtle::ConstantTimeEq;

type KeystoreType = String;
type KeystoreKey = String;
static KEYSTORE_TYPE: OnceLock<KeystoreType> = OnceLock::new();

type AesKey = Arc<String>;
type AesIv = Arc<String>;

pub struct AesCryptographer {
    key: AesKey,
    iv: AesIv,
}

type Aes256CfbEncoder = CfbEncryptor<Aes256>;
type Aes256CfbDecoder = CfbDecryptor<Aes256>;

impl AesCryptographer {
    pub fn new(key: AesKey, iv: AesIv) -> Self {
        Self { key, iv }
    }

    fn add_pkcs5_padding(data: &mut Vec<u8>, block_size: usize) {
        let padding_len = block_size - (data.len() % block_size);
        let padding_byte = padding_len as u8;
        data.extend(repeat(padding_byte).take(padding_len));
    }

    fn encrypt(&self, value: &String) -> StdR<String> {
        let cipher =
            match Aes256CfbEncoder::new_from_slices(self.key.as_bytes(), self.iv.as_bytes()) {
                Ok(c) => c,
                Err(e) => {
                    return RuntimeModelKernelErrorCode::AesCrypto
                        .msg(format!("Failed to create aes256 encoder, caused by {}.", e));
                }
            };
        let mut buf = value.as_bytes().to_vec();
        Self::add_pkcs5_padding(&mut buf, 16);
        cipher.encrypt(&mut buf);
        Ok(base64.encode(buf))
    }

    fn remove_pkcs5_padding(data: &mut Vec<u8>) -> Option<Vec<u8>> {
        let len = data.len();
        if len == 0 {
            return None;
        }

        let pad_len = data[len - 1] as usize;

        if pad_len == 0 || pad_len > 16 {
            return None;
        }

        if len < pad_len {
            return None;
        }

        let expected_padding = vec![pad_len as u8; pad_len];
        let actual_padding = &data[len - pad_len..];

        if expected_padding.ct_eq(actual_padding).unwrap_u8() == 0 {
            return None;
        }

        Some(data[..len - pad_len].to_vec())
    }

    fn decrypt(&self, value: &String) -> StdR<String> {
        let cipher =
            match Aes256CfbDecoder::new_from_slices(self.key.as_bytes(), self.iv.as_bytes()) {
                Ok(c) => c,
                Err(e) => {
                    return RuntimeModelKernelErrorCode::AesCrypto
                        .msg(format!("Failed to create aes256 decoder, caused by {}.", e));
                }
            };
        match base64.decode(&value) {
            Ok(mut buf) => {
                cipher.decrypt(&mut buf);
                let buf = if let Some(treated_buf) = Self::remove_pkcs5_padding(&mut buf) {
                    treated_buf
                } else {
                    buf
                };
                String::from_utf8(buf).map_err(|e| {
                    RuntimeModelKernelErrorCode::AesCrypto.err_with_msg(format!(
                        "Failed to create string by utf8 buffer, caused by {}.",
                        e
                    ))
                })
            }
            Err(e) => RuntimeModelKernelErrorCode::AesCrypto
                .msg(format!("Failed to create aes256 decoder, caused by {}.", e)),
        }
    }
}

static DEFAULT_PARAMS: OnceLock<(AesKey, AesIv)> = OnceLock::new();
static CRYPTOGRAPHERS: OnceLock<RwLock<HashMap<TenantId, HashMap<String, (AesKey, AesIv)>>>> =
    OnceLock::new();

pub struct AesCrypto {
    tenant_id: Arc<TenantId>,
}

type AesEncryptHead = String;

impl AesCrypto {
    fn init_default() -> (AesKey, AesIv) {
        (
            Arc::new("hWmZq4t7w9z$C&F)J@NcRfUjXn2r5u8x".to_string()),
            Arc::new("J@NcRfUjXn2r5u8x".to_string()),
        )
    }

    /// initialize aes params by given environment
    /// TIP call it at system startup
    pub fn init(envs: &EnvConfig) -> VoidR {
        let aes_key = envs.get_string("ENCRYPT_AES_KEY")?;
        let aes_iv = envs.get_string("ENCRYPT_AES_IV")?;
        let params = match (aes_key, aes_iv) {
            (Some(aes_key), Some(aes_iv)) => (Arc::new(aes_key), Arc::new(aes_iv)),
            (None, None) => Self::init_default(),
            (None, _) => StdErrCode::EnvInit.msg("Env variable[ENCRYPT_AES_KEY] not defined.")?,
            (_, None) => StdErrCode::EnvInit.msg("Env variable[ENCRYPT_AES_IV] not defined.")?,
        };

        DEFAULT_PARAMS
            .set(params)
            .or_else(|_| StdErrCode::EnvInit.msg("Failed to initialize aes key and iv."))
    }

    fn new(tenant_id: Arc<TenantId>) -> Self {
        Self { tenant_id }
    }

    fn get_encryption_head(value: &String) -> Option<String> {
        if value.starts_with("{AES") {
            if let Some(end_pos) = value.find('}') {
                let head = &value[..=end_pos];
                let suffix = &head[4..head.len() - 1];

                match suffix.len() {
                    0 => Some(head.to_string()), // {AES}
                    _ => {
                        // all chars are ascii digit,
                        // not 0, not starts with 0
                        if suffix.chars().all(|c| c.is_ascii_digit())
                            && suffix != "0"
                            && (suffix.len() == 1 || !suffix.starts_with('0'))
                        {
                            Some(head.to_string())
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn keystore_type() -> &'static KeystoreType {
        KEYSTORE_TYPE.get_or_init(|| FactorEncryptMethod::Aes256Pkcs5Padding.to_string())
    }

    fn keystore_key(head: &AesEncryptHead) -> Option<KeystoreKey> {
        let head_len = head.len();
        if head_len <= 5 {
            None
        } else {
            Some(head.as_str()[4..head_len - 1].to_string())
        }
    }

    fn load_params(&self, key: &Option<KeystoreKey>) -> StdR<Option<(AesKey, AesIv)>> {
        match KeyStoreService::find(Self::keystore_type(), key, self.tenant_id.deref())? {
            None => Ok(None),
            Some(mut params) => {
                let aes_key = match params.remove("key") {
                    Some(KeyStoreValue::Str(value)) => {
                        if value.len() != 32 {
                            return RuntimeModelKernelErrorCode::AesCrypto.msg(format!(
                                "Param[key]'s value[{}] for aes crypto must be 32 digits.",
                                value
                            ));
                        } else {
                            value
                        }
                    }
                    Some(value) => {
                        return RuntimeModelKernelErrorCode::AesCrypto.msg(format!(
                            "Param[key]'s value[{}] for aes crypto must be string.",
                            value
                        ));
                    }
                    _ => {
                        return RuntimeModelKernelErrorCode::AesCrypto
                            .msg("Param[key] for aes crypto not found.");
                    }
                };
                let aes_iv = match params.remove("iv") {
                    Some(KeyStoreValue::Str(value)) => {
                        if value.len() != 16 {
                            return RuntimeModelKernelErrorCode::AesCrypto.msg(format!(
                                "Param[iv]'s value[{}] for aes crypto must be 16 digits.",
                                value
                            ));
                        } else {
                            value
                        }
                    }
                    Some(value) => {
                        return RuntimeModelKernelErrorCode::AesCrypto.msg(format!(
                            "Param[iv]'s value[{}] for aes crypto must be string.",
                            value
                        ));
                    }
                    _ => {
                        return RuntimeModelKernelErrorCode::AesCrypto
                            .msg("Param[iv] for aes crypto not found.");
                    }
                };
                Ok(Some((Arc::new(aes_key), Arc::new(aes_iv))))
            }
        }
    }

    fn create_params(&self, key: &Option<KeystoreKey>) -> StdR<(AesKey, AesIv)> {
        if key.is_none() {
            let (aes_key, aes_iv) = DEFAULT_PARAMS.get_or_init(Self::init_default);
            Ok((aes_key.clone(), aes_iv.clone()))
        } else {
            todo!("implement create_params for AesCrypto")
        }
    }

    fn find_params_from_cache(&self, key: &Option<KeystoreKey>) -> StdR<Option<(AesKey, AesIv)>> {
        let guard = CRYPTOGRAPHERS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .read()
            .map_err(|e| {
                StdErrCode::RwLock
                    .err_with_msg(format!("Failed to get read lock, caused by {}.", e))
            })?;
        if let Some(tenant_map) = guard.get(self.tenant_id.deref()) {
            match key {
                Some(key) => {
                    if let Some((key, iv)) = tenant_map.get(key) {
                        return Ok(Some((key.clone(), iv.clone())));
                    }
                }
                _ => {
                    if let Some((key, iv)) = tenant_map.get("") {
                        return Ok(Some((key.clone(), iv.clone())));
                    }
                }
            }
        }
        Ok(None)
    }

    fn put_params_into_cache(
        &self,
        key: Option<KeystoreKey>,
        aes_key: AesKey,
        aes_iv: AesIv,
    ) -> VoidR {
        let mut guard = CRYPTOGRAPHERS
            .get_or_init(|| RwLock::new(HashMap::new()))
            .write()
            .map_err(|e| {
                StdErrCode::RwLock
                    .err_with_msg(format!("Failed to get write lock, caused by {}.", e))
            })?;

        if let Some(tenant_map) = guard.get_mut(self.tenant_id.deref()) {
            tenant_map
                .entry(key.unwrap_or(String::new()))
                .or_insert_with(|| (aes_key, aes_iv));
        } else {
            let mut map = HashMap::new();
            let _ = &map
                .entry(key.unwrap_or(String::new()))
                .or_insert_with(|| (aes_key, aes_iv));
            guard.insert(self.tenant_id.deref().clone(), map);
        };

        Ok(())
    }

    fn get_key_and_iv(&self, key: Option<KeystoreKey>) -> StdR<(AesKey, AesIv)> {
        if let Some((aes_key, aes_iv)) = self.find_params_from_cache(&key)? {
            return Ok((aes_key, aes_iv));
        }

        let (aes_key, aes_iv) = if let Some((aes_key, aes_iv)) = self.load_params(&key)? {
            (aes_key, aes_iv)
        } else {
            self.create_params(&key)?
        };
        self.put_params_into_cache(key, aes_key.clone(), aes_iv.clone())?;
        Ok((aes_key, aes_iv))
    }

    fn get_current_crypto() -> StdR<(AesCryptographer, AesEncryptHead)> {
        todo!("implement get_current_crypto for AesCrypto")
    }

    fn get_crypto(&self, head: &AesEncryptHead) -> StdR<AesCryptographer> {
        let (aes_key, aes_iv) = self.get_key_and_iv(Self::keystore_key(head))?;
        Ok(AesCryptographer::new(aes_key, aes_iv))
    }
}

impl Crypto for AesCrypto {
    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => Self::get_encryption_head(s).is_some(),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = CryptoUtils::value_to_str(value)? {
            let (cryptographer, mut head) = Self::get_current_crypto()?;
            let encrypted = cryptographer.encrypt(&str_value)?;
            head.push_str(&encrypted);
            Ok(Some(TopicDataValue::Str(head)))
        } else {
            Ok(None)
        }
    }

    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = CryptoUtils::value_to_str(value)? {
            if let Some(head) = Self::get_encryption_head(&str_value) {
                Ok(Some(TopicDataValue::Str(
                    self.get_crypto(&head)?.decrypt(
                        &str_value
                            .strip_prefix(&head)
                            .map(|s| s.to_string())
                            .unwrap_or(str_value),
                    )?,
                )))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

pub struct AesCryptoFinder;

impl AesCryptoFinder {
    pub fn get(tenant_id: &Arc<TenantId>) -> StdR<AesCrypto> {
        Ok(AesCrypto::new(tenant_id.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::AesCryptographer;
    use std::sync::Arc;

    // noinspection SpellCheckingInspection
    #[test]
    fn test() {
        let encryptor = AesCryptographer::new(
            Arc::new("0123456789abcdefghijklmnopqrstuv".to_string()),
            Arc::new("wxyz0123456789ab".to_string()),
        );
        assert_eq!(
            encryptor
                .encrypt(&"abc".to_string())
                .expect("encryption failed"),
            "{AES}wUcF6arwf6/5i9MWWTGeIA=="
        );
        assert_eq!(
            encryptor
                .decrypt(&"{AES}wUcF6arwf6/5i9MWWTGeIA==".to_string())
                .expect("decryption failed"),
            "abc"
        );
    }
}
