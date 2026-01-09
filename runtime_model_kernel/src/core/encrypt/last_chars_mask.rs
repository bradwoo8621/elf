use crate::{Crypto, CryptoUtils, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::TopicDataValue;

/// use [*] to mask trailing chars.
/// replace chars count (n) should follow given encrypt method.
/// - if given string chars count is less than n, then replace all to [*],
/// - if ascii digits chars count is less than n, then replace trailing n chars to [*],
/// - replace the trailing ascii digits chars to [*].
pub struct LastCharsMask {
    digits: usize,
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
        Self { digits: 3 }
    }

    pub fn last_6() -> Self {
        Self { digits: 6 }
    }

    fn replace_decimal_count(&self) -> usize {
        self.digits
    }
}

impl LastCharsMask {
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
            return CryptoUtils::n_asterisks(chars_count);
        }

        let decimal_count = CryptoUtils::get_ascii_digit_count(&value);

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

impl Crypto for LastCharsMask {
    /// always returns false.
    /// since even the last chars are [*], still do not know it is the original string or masked,
    /// thus treats anything as unencrypted.
    fn is_encrypted(&self, _value: &TopicDataValue) -> bool {
        false
    }

    // noinspection DuplicatedCode
    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        if let Some(str_value) = CryptoUtils::value_to_str(value)? {
            Ok(Some(TopicDataValue::Str(self.do_encrypt(str_value))))
        } else {
            Ok(None)
        }
    }

    /// always returns none, last chars mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Crypto, CryptoUtils, LastCharsMask};
    use elf_model::TopicDataValue;

    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [a***],
    /// - [12a3] -> [**a*],
    #[test]
    fn test() {
        let masker = LastCharsMask::last_3();
        assert_eq!(
            "**",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab".to_string())))
        );
        assert_eq!(
            "***",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("abc".to_string())))
        );
        assert_eq!(
            "a***",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab1c".to_string())))
        );
        assert_eq!(
            "**a*",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("12a3".to_string())))
        );
    }
}
