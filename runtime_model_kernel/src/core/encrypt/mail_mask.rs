use crate::{Crypto, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::TopicDataValue;

/// use [*****] to mask chars before [@].
pub struct MailMask;

impl MailMask {
    pub fn new() -> Self {
        Self
    }
}

impl Crypto for MailMask {
    fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        match value {
            TopicDataValue::Str(s) => s.starts_with("*****@"),
            _ => false,
        }
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Str(s) => {
                if let Some(index) = s.find('@') {
                    let mut s = s.clone();
                    let byte_index = s.char_indices().nth(index).unwrap().0;
                    s.replace_range(0..byte_index, "*****@");
                    Ok(Some(TopicDataValue::Str(s)))
                } else {
                    Ok(None)
                }
            }
            TopicDataValue::None => Ok(None),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                "Mail mask doesn't support value[{}], it must be a string or none.",
                value
            )),
        }
    }

    /// always returns none, last chars mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}
