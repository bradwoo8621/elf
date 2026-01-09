use crate::{Crypto, CryptoUtils};
use elf_base::StdR;
use elf_model::TopicDataValue;
use md5::compute;

pub struct Md5Crypto;

impl Md5Crypto {
    pub fn new() -> Self {
        Self
    }
}

impl Crypto for Md5Crypto {
    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("{MD5}"),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = CryptoUtils::value_to_str(value)? {
            let encrypted = format!("{{MD5}}{:x}", &compute(str_value.as_bytes()));
            Ok(Some(TopicDataValue::Str(encrypted)))
        } else {
            Ok(None)
        }
    }

    ///  md5 cannot be decrypted, remove prefix only
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Str(s) => {
                if let Some(decrypted) = s.strip_prefix("{MD5}") {
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
    use crate::{Crypto, CryptoUtils, Md5Crypto};
    use elf_model::TopicDataValue;

    #[test]
    fn test() {
        let encryptor = Md5Crypto::new();
        assert_eq!(
            CryptoUtils::get_str(encryptor.encrypt(&TopicDataValue::Str("abc".to_string()))),
            "{MD5}900150983cd24fb0d6963f7d28e17f72"
        )
    }
}
