use crate::{Crypto, CryptoUtils};
use elf_base::StdR;
use elf_model::TopicDataValue;
use hex::encode as hex_encode;
use sha2::{Digest, Sha256};

pub struct Sha256Crypto;

impl Sha256Crypto {
    pub fn new() -> Self {
        Self
    }
}

impl Crypto for Sha256Crypto {
    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("{SHA256}"),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = CryptoUtils::value_to_str(value)? {
            let mut sha256 = Sha256::new();
            sha256.update(str_value.as_bytes());
            let value = sha256.finalize();
            let encrypted = format!("{{SHA256}}{}", hex_encode(value));
            Ok(Some(TopicDataValue::Str(encrypted)))
        } else {
            Ok(None)
        }
    }

    ///  md5 cannot be decrypted, remove prefix only
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Str(s) => {
                if let Some(decrypted) = s.strip_prefix("{SHA256}") {
                    Ok(Some(TopicDataValue::Str(decrypted.to_string())))
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
    use crate::{Crypto, CryptoUtils, Sha256Crypto};
    use elf_model::TopicDataValue;

    #[test]
    fn test() {
        let encryptor = Sha256Crypto::new();
        assert_eq!(
            CryptoUtils::get_str(encryptor.encrypt(&TopicDataValue::Str("abc".to_string()))),
            "{SHA256}ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        )
    }
}
