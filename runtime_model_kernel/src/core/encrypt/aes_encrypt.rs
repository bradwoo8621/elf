use crate::{Encryptor, StrEncryptor};
use aes::Aes256;
use base64::{engine::general_purpose::STANDARD as base64, Engine};
use cfb_mode::{
    cipher::{AsyncStreamCipher, KeyIvInit}, Decryptor as CfbDecryptor,
    Encryptor as CfbEncryptor,
};
use std::iter::repeat;
use subtle::ConstantTimeEq;
use watchmen_base::StdR;
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

pub struct AesEncrypt {
    key: String,
    iv: String,
    method: FactorEncryptMethod,
}

type Aes256CfbEncoder = CfbEncryptor<Aes256>;
type Aes256CfbDecoder = CfbDecryptor<Aes256>;

impl AesEncrypt {
    pub fn new(key: String, iv: String) -> Self {
        Self {
            key,
            iv,
            method: FactorEncryptMethod::Aes256Pkcs5Padding,
        }
    }

    fn add_pkcs5_padding(data: &mut Vec<u8>, block_size: usize) {
        let padding_len = block_size - (data.len() % block_size);
        let padding_byte = padding_len as u8;
        data.extend(repeat(padding_byte).take(padding_len));
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

    fn do_decrypt(&self, value: &String) -> Option<String> {
        let cipher =
            match Aes256CfbDecoder::new_from_slices(self.key.as_bytes(), self.iv.as_bytes()) {
                Ok(c) => c,
                Err(_e) => {
                    // eprintln!("{}", _e);
                    return None;
                }
            };
        if let Some(value) = value.strip_prefix("{AES}") {
            if let Ok(mut buf) = base64.decode(&value) {
                cipher.decrypt(&mut buf);
                if let Some(buf) = Self::remove_pkcs5_padding(&mut buf) {
                    if let Ok(s) = String::from_utf8(buf) {
                        return Some(s);
                    }
                }
            }
        }
        None
    }
}

impl StrEncryptor for AesEncrypt {
    fn do_encrypt(&self, value: String) -> String {
        let cipher =
            match Aes256CfbEncoder::new_from_slices(self.key.as_bytes(), self.iv.as_bytes()) {
                Ok(c) => c,
                Err(_e) => {
                    // eprintln!("{}", _e);
                    return value;
                }
            };
        let mut buf = value.as_bytes().to_vec();
        Self::add_pkcs5_padding(&mut buf, 16);
        cipher.encrypt(&mut buf);
        format!("{{AES}}{}", base64.encode(buf))
    }
}

impl Encryptor for AesEncrypt {
    fn method(&self) -> &FactorEncryptMethod {
        &self.method
    }

    fn accept(&self, method: &FactorEncryptMethod) -> bool {
        match method {
            FactorEncryptMethod::Aes256Pkcs5Padding => true,
            _ => false,
        }
    }

    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("{AES}"),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        StrEncryptor::encrypt(self, value)
    }

    ///  md5 cannot be decrypted, remove prefix only
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Str(s) => {
                if s.starts_with("{AES}") {
                    Ok(self.do_decrypt(s).map(|s| TopicDataValue::Str(s)))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AesEncrypt, Encryptor, EncryptorUtils, StrEncryptor};
    use watchmen_model::TopicDataValue;

    #[test]
    fn test() {
        let encryptor = AesEncrypt::new(
            "0123456789abcdefghijklmnopqrstuv".to_string(),
            "wxyz0123456789ab".to_string(),
        );
        assert_eq!(
            EncryptorUtils::get_str(StrEncryptor::encrypt(
                &encryptor,
                &TopicDataValue::Str("abc".to_string())
            )),
            "{AES}wUcF6arwf6/5i9MWWTGeIA=="
        );
        assert_eq!(
            EncryptorUtils::get_str(encryptor.decrypt(&TopicDataValue::Str(
                "{AES}wUcF6arwf6/5i9MWWTGeIA==".to_string()
            ))),
            "abc"
        );
    }
}
