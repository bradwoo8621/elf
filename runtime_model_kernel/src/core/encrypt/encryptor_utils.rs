use crate::RuntimeModelKernelErrorCode;
use watchmen_base::{ErrorCode, StdR};
use watchmen_model::TopicDataValue;

pub struct EncryptorUtils;

impl EncryptorUtils {
    /// map, vec -> raise error,
    /// none, empty string -> [Ok(None)],
    /// not empty string -> [Ok(Some(str))],
    /// other -> [Ok(Some(value.to_string()))].
    pub fn value_to_str(value: &TopicDataValue) -> StdR<Option<String>> {
        let str_value = match value {
            TopicDataValue::Str(s) => {
                if s.is_empty() {
                    return Ok(None);
                } else {
                    s.clone()
                }
            }
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
    pub fn get_str(value: StdR<Option<TopicDataValue>>) -> String {
        let value = value
            .expect("Failed to get value from StdR<>.")
            .expect("Failed to get value from Option<>.");
        match value {
            TopicDataValue::Str(str) => str,
            _ => panic!("Value from TopicDataValue is not string."),
        }
    }

    #[cfg(test)]
    pub fn get_date_str(value: StdR<Option<TopicDataValue>>) -> String {
        let value = value
            .expect("Failed to get value from StdR<>.")
            .expect("Failed to get value from Option<>.");
        match value {
            TopicDataValue::Date(d) => d.format("%Y-%m-%d").to_string(),
            TopicDataValue::DateTime(dt) => dt.format("%Y-%m-%d").to_string(),
            _ => panic!("Value from TopicDataValue is not date or datetime."),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn test() {
        assert_eq!(
            NaiveDateTime::parse_from_str("2026-01-08 01:02:03", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .to_string(),
            "2026-01-08 01:02:03"
        );
        assert_eq!(
            NaiveDate::parse_from_str("2026-01-08", "%Y-%m-%d")
                .unwrap()
                .to_string(),
            "2026-01-08"
        );
        assert_eq!(
            NaiveTime::parse_from_str("01:02:03", "%H:%M:%S")
                .unwrap()
                .to_string(),
            "01:02:03"
        );
    }
}
