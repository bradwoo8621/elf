use crate::{Crypto, CryptoUtils, KeyStoreService, RuntimeModelKernelErrorCode};
use aes::Aes256;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use cfb_mode::{
    cipher::{AsyncStreamCipher, KeyIvInit}, Decryptor as CfbDecryptor,
    Encryptor as CfbEncryptor,
};
use std::iter::repeat;
use std::ops::Deref;
use std::sync::Arc;
use subtle::ConstantTimeEq;
use watchmen_base::{ErrorCode, StdR};
use watchmen_model::{FactorEncryptMethod, KeyStoreValue, TenantId, TopicDataValue};

pub struct AesCryptographer {
    key: String,
    iv: String,
}

type Aes256CfbEncoder = CfbEncryptor<Aes256>;
type Aes256CfbDecoder = CfbDecryptor<Aes256>;

impl AesCryptographer {
    pub fn new(key: String, iv: String) -> Self {
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

pub struct AesCrypto {
    tenant_id: Arc<TenantId>,
}

impl AesCrypto {
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

    fn get_current_crypto() -> StdR<AesCryptographer> {
        todo!("implement get_current_crypto for AesCrypto")
    }

    fn get_crypto(&self, head: &String) -> StdR<AesCryptographer> {
        let head_len = head.len();
        let key = if head_len <= 5 {
            None
        } else {
            Some(head.as_str()[4..head_len - 1].to_string())
        };
        // TODO cache!
        let params = KeyStoreService::find(
            &FactorEncryptMethod::Aes256Pkcs5Padding.to_string(),
            key,
            self.tenant_id.deref(),
        )?;

        let aes_key = match params.get("key") {
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
        let aes_iv = match params.get("iv") {
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

        Ok(AesCryptographer::new(aes_key.clone(), aes_iv.clone()))
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
            Ok(Some(TopicDataValue::Str(
                Self::get_current_crypto()?.encrypt(&str_value)?,
            )))
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

    // noinspection SpellCheckingInspection
    #[test]
    fn test() {
        let encryptor = AesCryptographer::new(
            "0123456789abcdefghijklmnopqrstuv".to_string(),
            "wxyz0123456789ab".to_string(),
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
