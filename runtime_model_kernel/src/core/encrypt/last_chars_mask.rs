use crate::{Encryptor, EncryptorUtils, RuntimeModelKernelErrorCode, StrEncryptor};
use watchmen_base::{ErrorCode, StdR};
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

/// use [*] to mask trailing chars
pub struct LastCharsMask {
    method: FactorEncryptMethod,
}

impl LastCharsMask {
    pub fn new(digits: usize) -> StdR<Self> {
        match digits {
            3 => Ok(Self::last_3()),
            6 => Ok(Self::last_6()),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!("Given digits[{}] is not supported by last chars mask, only 3 or 6 digits is supported.", digits))
        }
    }

    pub fn last_3() -> Self {
        Self {
            method: FactorEncryptMethod::MaskLast3,
        }
    }

    pub fn last_6() -> Self {
        Self {
            method: FactorEncryptMethod::MaskLast6,
        }
    }

    fn replace_decimal_count(&self) -> usize {
        match self.method {
            FactorEncryptMethod::MaskLast3 => 3,
            FactorEncryptMethod::MaskLast6 => 6,
            // default 3
            _ => 3,
        }
    }
}

impl StrEncryptor for LastCharsMask {
    /// when given str
    /// - length is less than digits, all chars replaced with [*],
    /// - if there is no enough ascii digit char([0-9]) in given str, replace the trailing digits to [*],
    /// - replace the trailing ascii digit chars([0-9]) to [*].
    ///
    /// for example, digits is 3
    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [a***],
    /// - [12a3] -> [**a*],
    fn do_encrypt(&self, mut value: String) -> String {
        let replace_decimal_count = self.replace_decimal_count();
        let chars_count = value.chars().count();
        if chars_count <= replace_decimal_count {
            return EncryptorUtils::n_asterisks(chars_count);
        }

        let decimal_count = EncryptorUtils::get_ascii_digit_count(&value);

        if decimal_count < replace_decimal_count {
            let replace_start = value
                .char_indices()
                .nth(chars_count - replace_decimal_count)
                .unwrap()
                .0;
            for offset in 0..replace_decimal_count {
                let index = replace_start + offset;
                value.replace_range(index..index + 1, "*");
            }
        } else {
            let mut indices = vec![];
            let mut remain_decimal_count = replace_decimal_count;
            let mut index = chars_count;
            for ch in value.chars().rev() {
                index -= ch.len_utf8();
                if remain_decimal_count > 0 && ch.is_ascii_digit() {
                    indices.push(index);
                    remain_decimal_count -= 1;
                }
            }
            for index in indices {
                value.replace_range(index..index + 1, "*");
            }
        }

        value
    }
}

impl Encryptor for LastCharsMask {
    fn method(&self) -> &FactorEncryptMethod {
        &self.method
    }

    fn accept(&self, method: &FactorEncryptMethod) -> bool {
        match method {
            FactorEncryptMethod::MaskLast3 | FactorEncryptMethod::MaskLast6 => true,
            _ => false,
        }
    }

    /// always returns false.
    /// since even the last chars are [*], still do not know it is the original string or masked,
    /// thus treats anything as unencrypted.
    fn is_encrypted(&self, _value: &TopicDataValue) -> bool {
        false
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        StrEncryptor::encrypt(self, value)
    }

    /// always returns none, last chars mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encryptor, EncryptorUtils, LastCharsMask};
    use watchmen_model::TopicDataValue;

    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [a***],
    /// - [12a3] -> [**a*],
    #[test]
    fn test() {
        let masker = LastCharsMask::last_3();
        assert_eq!(
            "**",
            EncryptorUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab".to_string())))
        );
        assert_eq!(
            "***",
            EncryptorUtils::get_str(masker.encrypt(&TopicDataValue::Str("abc".to_string())))
        );
        assert_eq!(
            "a***",
            EncryptorUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab1c".to_string())))
        );
        assert_eq!(
            "**a*",
            EncryptorUtils::get_str(masker.encrypt(&TopicDataValue::Str("12a3".to_string())))
        );
    }
}
