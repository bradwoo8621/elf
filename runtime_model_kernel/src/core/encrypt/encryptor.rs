use crate::EncryptorUtils;
use watchmen_base::StdR;
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

pub trait Encryptor {
    fn key_type(&self) -> String {
        self.method().to_string()
    }

    fn method(&self) -> &FactorEncryptMethod;

    fn accept(&self, method: &FactorEncryptMethod) -> bool;

    /// return false when
    /// - not encrypted,
    /// - or given value not accepted by this encryptor.
    fn is_encrypted(&self, value: &TopicDataValue) -> bool;

    /// returns none when no encryption applied
    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>>;

    /// returns none when no decryption applied
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>>;
}

pub trait StrEncryptor {
    fn do_encrypt(&self, value: String) -> String;

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = EncryptorUtils::value_to_str(value)? {
            Ok(Some(TopicDataValue::Str(self.do_encrypt(str_value))))
        } else {
            Ok(None)
        }
    }
}
