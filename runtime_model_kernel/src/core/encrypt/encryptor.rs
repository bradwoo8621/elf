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
