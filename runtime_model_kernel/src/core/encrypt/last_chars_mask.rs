use crate::{Encryptor, RuntimeModelKernelErrorCode};
use watchmen_base::{ErrorCode, StdR};
use watchmen_model::{FactorEncryptMethod, TopicDataValue};

/// use [*] to mask char
pub struct LastCharsMask {
    digits: usize,
    method: FactorEncryptMethod,
}

impl LastCharsMask {
    pub fn new(digits: usize) -> StdR<Self> {
        match digits {
            3 => Ok(Self { digits, method: FactorEncryptMethod::MaskLast3 }),
            6 => Ok(Self { digits, method: FactorEncryptMethod::MaskLast6 }),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!("Given digits[{}] is not supported by last chars mask, only 3 or 6 digits is supported.", digits))
        }
    }

    /// when given str
    /// - length is less than digits, all chars replaced with [*],
    /// - if there is no enough ascii digit char([0-9]) in given str, replace the tailing digits to [*],
    /// - replace the tailing ascii digit chars([0-9]) to [*].
    ///
    /// for example, digits is 3
    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [a***],
    /// - [12a3] -> [**a*],
    fn do_encrypt(&self, value: &str) -> String {
        let length = value.chars().count();
        if length <= self.digits {
            return "*".repeat(length);
        }

        let mut decimal_count = 0;
        for ch in value.chars() {
            if ch.is_ascii_digit() {
                decimal_count += 1;
            }
        }

        if decimal_count < self.digits {
            let mut result = String::with_capacity(length);
            result.push_str(&value[0..length - self.digits]);
            result.extend(std::iter::repeat('*').take(self.digits));
            return result;
        }

        let mut result = String::with_capacity(length);
        let mut remaining_digits = self.digits;
        for ch in value.chars().rev() {
            if remaining_digits > 0 && ch.is_ascii_digit() {
                result.push('*');
                remaining_digits -= 1;
            } else {
                result.push(ch);
            }
        }
        result.chars().rev().collect()
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
        let str_value = match value {
            TopicDataValue::Str(s) => s.clone(),
            TopicDataValue::Num(n) => n.to_string(),
            TopicDataValue::Bool(b) => b.to_string(),
            TopicDataValue::DateTime(dt) => dt.to_string(),
            TopicDataValue::Date(d) => d.to_string(),
            TopicDataValue::Time(t) => t.to_string(),
            TopicDataValue::Map(map) => {
                return RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                    "Last chars mask doesn't support map value[{}].",
                    TopicDataValue::map_to_display(map)
                ));
            }
            TopicDataValue::Vec(vec) => {
                return RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                    "Last chars mask doesn't support vec value[{}].",
                    TopicDataValue::vec_to_display(vec)
                ));
            }
            TopicDataValue::None => String::default(),
        };

        Ok(Some(TopicDataValue::Str(
            self.do_encrypt(str_value.as_str()),
        )))
    }

    /// always returns none, last chars mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encryptor, LastCharsMask};
    use watchmen_base::StdR;
    use watchmen_model::TopicDataValue;

    fn stringify(value: StdR<Option<TopicDataValue>>) -> String {
        let value = value
            .expect("Failed to get value from StdR<>.")
            .expect("Failed to get value from Option<>.");
        match value {
            TopicDataValue::Str(str) => str,
            _ => panic!("Value from TopicDataValue is not string."),
        }
    }

    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [a***],
    /// - [12a3] -> [**a*],
    #[test]
    fn test() {
        let masker = LastCharsMask::new(3).expect("Failed to create masker");
        assert_eq!(
            "**",
            stringify(masker.encrypt(&TopicDataValue::Str("ab".to_string())))
        );
        assert_eq!(
            "***",
            stringify(masker.encrypt(&TopicDataValue::Str("abc".to_string())))
        );
        assert_eq!(
            "a***",
            stringify(masker.encrypt(&TopicDataValue::Str("ab1c".to_string())))
        );
        assert_eq!(
            "**a*",
            stringify(masker.encrypt(&TopicDataValue::Str("12a3".to_string())))
        );
    }
}
