use elf_base::StdR;
use elf_model::TopicDataValue;

pub trait Crypto {
    /// return false when
    /// - not encrypted,
    /// - or given value not accepted by this encryptor.
    fn is_encrypted(&self, value: &TopicDataValue) -> bool;

    /// returns none when no encryption applied
    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>>;

    /// returns none when no decryption applied
    fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>>;
}
