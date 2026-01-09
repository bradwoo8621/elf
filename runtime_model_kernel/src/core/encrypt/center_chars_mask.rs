use crate::{Crypto, CryptoUtils, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::TopicDataValue;

/// use [*] to mask center chars
/// replace chars count (n) should follow given encrypt method.
/// - if given string chars count is less than n, then replace all to [*],
/// - if ascii digits chars count is less than n, then replace center n chars to [*],
///   center chars means:
///   - for chars count is even, they are [len / 2 - 2] (3rd), [len / 2 - 1] (1st), [len / 2] (2nd),
///   - for chars count is odd, they are [(len - 1) / 2 - 1] (3rd), [(len - 1) / 2] (1st), [(len - 1) / 2 + 1] (2nd),
/// - replace the center ascii digits chars to [*],
///   center chars counting is same as above, but only ascii digits chars are counted in.
pub struct CenterCharsMask {
    digits: usize,
}

impl CenterCharsMask {
    pub fn new(digits: usize) -> StdR<Self> {
        match digits {
            3 => Ok(Self::center_3()),
            5 => Ok(Self::center_5()),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!("Given digits[{}] is not supported by center chars mask, only 3 or 5 digits is supported.", digits))
        }
    }

    pub fn center_3() -> Self {
        Self { digits: 3 }
    }

    pub fn center_5() -> Self {
        Self { digits: 5 }
    }

    fn replace_decimal_count(&self) -> usize {
        self.digits
    }
}

struct CenterDecimalReplacePosition {
    char_index: usize,
    byte_index: usize,
    end: bool,
}

struct CenterDecimalReplacePositions {
    backward: CenterDecimalReplacePosition,
    forward: CenterDecimalReplacePosition,
}

enum CenterDecimalReplaceDirection {
    Backward,
    Forward,
}

/// TIP: It is not the same as the Python version.
/// Since this mask cannot be decrypted, it means the information has been lost anyway.
/// No matter what the mask implementation is, it cannot change this fact.
/// Therefore, changing the implementation will not be a problem.
///
/// This implementation is changed to start from the middle position [len / 2] of the given string.
/// First, it covers a digit on the left, then a digit on the right,
/// and repeats this process in a loop until all the required number of digit characters are covered.
///
/// - chars count must be greater than or equal decimal count,
/// - decimal count must be greater than or equal [digits].
struct CenterDecimalReplacer {
    str: String,
    chars: Vec<char>,
    /// chars count of string
    chars_count: usize,
    /// decimal char count to replace
    replace_count: usize,

    positions: CenterDecimalReplacePositions,
    direction: CenterDecimalReplaceDirection,
}

impl CenterDecimalReplacer {
    /// refer to the only caller [CenterCharsMask::replace_ascii_digit_with_asterisks].
    fn replace(str: String, replace_count: usize) -> String {
        let chars = str.chars().collect::<Vec<char>>();
        let chars_count = chars.len();
        let backward_char_index = (chars_count - 1) / 2;
        let forward_char_index = backward_char_index + 1;
        let mut backward_byte_index: usize = 0;
        for index in 0..backward_char_index {
            let ch = chars.get(index).unwrap();
            backward_byte_index += ch.len_utf8();
        }
        let start_char = chars.get(backward_char_index).unwrap();
        let forward_byte_index = backward_byte_index + start_char.len_utf8();

        Self {
            str,
            chars,
            chars_count,
            replace_count,
            positions: CenterDecimalReplacePositions {
                backward: CenterDecimalReplacePosition {
                    char_index: backward_char_index,
                    byte_index: backward_byte_index,
                    end: false,
                },
                forward: CenterDecimalReplacePosition {
                    char_index: forward_char_index,
                    byte_index: forward_byte_index,
                    end: false,
                },
            },
            direction: CenterDecimalReplaceDirection::Backward,
        }
        .do_replace()
    }

    fn get_current_position(&self) -> &CenterDecimalReplacePosition {
        match &self.direction {
            CenterDecimalReplaceDirection::Backward => &self.positions.backward,
            CenterDecimalReplaceDirection::Forward => &self.positions.forward,
        }
    }

    fn move_one_char_at_same_direction(&mut self) {
        match &self.direction {
            CenterDecimalReplaceDirection::Backward => {
                let position = &mut self.positions.backward;
                let char_index = position.char_index;
                if char_index == 0 {
                    // it is the first char
                    position.end = true
                } else {
                    // move to previous char
                    let char = self.chars.get(char_index - 1).unwrap();
                    position.char_index -= 1;
                    position.byte_index -= char.len_utf8();
                }
            }
            CenterDecimalReplaceDirection::Forward => {
                let position = &mut self.positions.forward;
                let char_index = position.char_index;
                if char_index == self.chars_count - 1 {
                    // it is the last char
                    position.end = true
                } else {
                    // move to next char
                    let char = self.chars.get(char_index).unwrap();
                    position.char_index += 1;
                    position.byte_index += char.len_utf8();
                }
            }
        }
    }

    fn end_current_direction(&mut self) {
        match &self.direction {
            CenterDecimalReplaceDirection::Backward => {
                self.positions.backward.end = true;
            }
            CenterDecimalReplaceDirection::Forward => {
                self.positions.forward.end = true;
            }
        }
    }

    /// returns true when at least one direction is not end
    fn switch_to_another_direction(&mut self) -> bool {
        match &self.direction {
            CenterDecimalReplaceDirection::Backward => {
                if self.positions.forward.end != true {
                    self.direction = CenterDecimalReplaceDirection::Forward;
                    true
                } else {
                    !self.positions.backward.end
                }
            }
            CenterDecimalReplaceDirection::Forward => {
                if self.positions.backward.end != true {
                    self.direction = CenterDecimalReplaceDirection::Backward;
                    true
                } else {
                    !self.positions.forward.end
                }
            }
        }
    }

    fn do_replace(mut self) -> String {
        while self.replace_count > 0 {
            let position = self.get_current_position();
            if let Some(char) = self.chars.get(position.char_index) {
                if char.is_ascii_digit() {
                    self.str
                        .replace_range(position.byte_index..position.byte_index + 1, "*");
                    self.replace_count -= 1;
                }
                self.move_one_char_at_same_direction();
            } else {
                self.end_current_direction();
            }
            if !self.switch_to_another_direction() {
                break;
            }
        }

        self.str
    }
}

impl CenterCharsMask {
    /// chars count must be greater than or equal [digits].
    fn replace_with_asterisks(&self, mut value: String, chars_count: usize) -> String {
        let replace_decimal_count = self.replace_decimal_count();
        let remain_char_count = chars_count.saturating_sub(replace_decimal_count);
        let chars_count_from_start = remain_char_count / 2;
        let mut start_index: usize = 0;
        for ch in value.chars().take(chars_count_from_start) {
            start_index += ch.len_utf8();
        }
        value.replace_range(
            start_index..start_index + replace_decimal_count,
            &CryptoUtils::n_asterisks(replace_decimal_count),
        );
        value
    }
}

impl CenterCharsMask {
    /// when given str
    /// - length is less than digits, all chars replaced with [*],
    /// - if there is no enough ascii digit char([0-9]) in given str, replace the center digits to [*],
    /// - replace the center ascii digit chars([0-9]) to [*].
    ///
    /// for example, digits is 3
    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [***c],
    /// - [ab1cd] -> [a***c],
    /// - [12a3] -> [**a*],
    /// - [12a34] -> [**a*4],
    /// - [123a456] -> [1**a*56]
    fn do_encrypt(&self, value: String) -> String {
        let replace_decimal_count = self.replace_decimal_count();
        let chars_count = value.chars().count();
        if chars_count <= replace_decimal_count {
            return CryptoUtils::n_asterisks(chars_count);
        }

        let decimal_count = CryptoUtils::get_ascii_digit_count(&value);
        if decimal_count < replace_decimal_count {
            self.replace_with_asterisks(value, chars_count)
        } else {
            CenterDecimalReplacer::replace(value, replace_decimal_count)
        }
    }
}

impl Crypto for CenterCharsMask {
    /// always returns false.
    /// since even the center chars are [*], still do not know it is the original string or masked,
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

    /// always returns none, center chars mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CenterCharsMask, Crypto, CryptoUtils};
    use elf_model::TopicDataValue;

    /// - [ab] -> [**],
    /// - [abc] -> [***],
    /// - [ab1c] -> [***c],
    /// - [ab1cd] -> [a***c],
    /// - [12a3] -> [**a*],
    /// - [12a34] -> [1*a**],
    /// - [123a456] -> [12*a**6]
    #[test]
    fn test() {
        let masker = CenterCharsMask::center_3();
        assert_eq!(
            "**",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab".to_string())))
        );
        assert_eq!(
            "***",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("abc".to_string())))
        );
        assert_eq!(
            "***c",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("ab1c".to_string())))
        );
        assert_eq!(
            "**a*",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("12a3".to_string())))
        );
        assert_eq!(
            "1*a**",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("12a34".to_string())))
        );
        assert_eq!(
            "12*a**6",
            CryptoUtils::get_str(masker.encrypt(&TopicDataValue::Str("123a456".to_string())))
        );
    }
}
