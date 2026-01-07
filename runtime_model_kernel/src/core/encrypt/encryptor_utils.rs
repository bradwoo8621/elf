use crate::RuntimeModelKernelErrorCode;
use watchmen_base::{ErrorCode, StdR};
use watchmen_model::TopicDataValue;

pub struct EncryptorUtils;

impl EncryptorUtils {
    pub fn value_to_str(value: &TopicDataValue) -> StdR<Option<String>> {
        let str_value = match value {
            TopicDataValue::Str(s) => s.clone(),
            TopicDataValue::Num(n) => n.to_string(),
            TopicDataValue::Bool(b) => b.to_string(),
            TopicDataValue::DateTime(dt) => dt.to_string(),
            TopicDataValue::Date(d) => d.to_string(),
            TopicDataValue::Time(t) => t.to_string(),
            TopicDataValue::Map(map) => {
                return RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                    "Center chars mask doesn't support map value[{}].",
                    TopicDataValue::map_to_display(map)
                ));
            }
            TopicDataValue::Vec(vec) => {
                return RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                    "Center chars mask doesn't support vec value[{}].",
                    TopicDataValue::vec_to_display(vec)
                ));
            }
            TopicDataValue::None => return Ok(None),
        };

        Ok(Some(str_value))
    }

    pub fn n_asterisks(n: usize) -> String {
        "*".repeat(n)
    }

    pub fn get_ascii_digit_count(value: &String) -> usize {
        let mut decimal_count = 0;
        for ch in value.chars() {
            if ch.is_ascii_digit() {
                decimal_count += 1;
            }
        }
        decimal_count
    }

    #[cfg(test)]
    pub fn stringify(value: StdR<Option<TopicDataValue>>) -> String {
        let value = value
            .expect("Failed to get value from StdR<>.")
            .expect("Failed to get value from Option<>.");
        match value {
            TopicDataValue::Str(str) => str,
            _ => panic!("Value from TopicDataValue is not string."),
        }
    }
}
