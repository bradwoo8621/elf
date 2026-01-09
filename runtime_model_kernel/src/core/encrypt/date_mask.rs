use crate::{Crypto, RuntimeModelKernelErrorCode};
use chrono::Datelike;
use elf_base::{ErrorCode, LooseDateFormatter, StdR};
use elf_model::TopicDataValue;

/// - mask month to 1,
/// - mask date of month to 1.
/// - for string value, try to parse them to date/datetime, and mask, and format to string.
pub struct DateMask {
    month: bool,
    date: bool,
}

impl DateMask {
    pub fn new(month: bool, day_of_month: bool) -> StdR<Self> {
        match (month, day_of_month) {
            (true, true) => Ok(Self::month_and_day()),
            (true, false) => Ok(Self::month()),
            (false, true) => Ok(Self::day_of_month()),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport
                .msg("For date mask, one of month or day of month must be true, or both true."),
        }
    }

    pub fn month_and_day() -> Self {
        Self {
            month: true,
            date: true,
        }
    }

    pub fn month() -> Self {
        Self {
            month: true,
            date: false,
        }
    }

    pub fn day_of_month() -> Self {
        Self {
            month: false,
            date: true,
        }
    }
}

impl DateMask {
    fn mask_date<D: Datelike>(&self, date: &D) -> D {
        match (self.month, self.date) {
            (true, false) => date.with_month(1).unwrap(),
            (false, true) => date.with_day(1).unwrap(),
            (true, true) | _ => date.with_month(1).unwrap().with_day(1).unwrap(),
        }
    }
}

impl Crypto for DateMask {
    /// always returns false.
    /// since mask part is changed to another valid random value, still do not know it is the original date or masked,
    /// thus treats anything as unencrypted.
    fn is_encrypted(&self, _value: &TopicDataValue) -> bool {
        false
    }

    fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        match value {
            TopicDataValue::Date(d) => Ok(Some(TopicDataValue::Date(self.mask_date(d)))),
            TopicDataValue::DateTime(dt) => Ok(Some(TopicDataValue::DateTime(self.mask_date(dt)))),
            TopicDataValue::Str(s) => {
                if s.is_empty() {
                    return Ok(None);
                }

                if let Ok((dt, format)) = LooseDateFormatter::parse_datetime_and_format(&s) {
                    // given string can be parsed to a datetime
                    let masked = self.mask_date(&dt);
                    let masked_str = masked.format(&format).to_string();
                    if masked_str.len() == s.len() {
                        return Ok(Some(TopicDataValue::Str(masked_str)));
                    }

                    let mut s = s.clone();
                    let s_chars = s.chars().collect::<Vec<char>>();
                    let mut byte_index = 0;
                    let mut char_index_of_masked = 0;
                    for ch in s_chars {
                        if ch.is_ascii_digit() || ch == '+' {
                            // is part of masked, use char from masked string
                            s.replace_range(byte_index..byte_index + 1, masked_str.get(char_index_of_masked..char_index_of_masked + 1).unwrap());
                            char_index_of_masked +=1;
                        }
                        byte_index += ch.len_utf8();
                    }
                    Ok(Some(TopicDataValue::Str(s)))
                } else {
                    RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                        "Date mask doesn't support string value[{}], it must can be parsed to date/datetime.",
                        value
                    ))
                }
            },
            TopicDataValue::None => Ok(None),
            _ => RuntimeModelKernelErrorCode::EncryptNotSupport.msg(format!(
                "Date mask doesn't support value[{}], it must be a date, datetime, date/datetime in string or none.",
                value
            )),
        }
    }

    /// always returns none, date mask cannot be decrypted.
    fn decrypt(&self, _value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Crypto, CryptoUtils, DateMask};
    use chrono::{NaiveDate, NaiveTime};
    use elf_model::TopicDataValue;

    #[test]
    fn test() {
        let mask = DateMask::month_and_day();
        let date = NaiveDate::from_ymd_opt(2025, 2, 8).unwrap();
        let time = NaiveTime::default();
        let datetime = date.and_time(time);
        assert_eq!(
            CryptoUtils::get_date_str(mask.encrypt(&TopicDataValue::DateTime(datetime.clone()))),
            "2025-01-01"
        );
        assert_eq!(
            CryptoUtils::get_date_str(mask.encrypt(&TopicDataValue::Date(date.clone()))),
            "2025-01-01"
        );
        assert_eq!(
            CryptoUtils::get_str(mask.encrypt(&TopicDataValue::Str("2025-02-08 abc".to_string()))),
            "2025-01-01 abc"
        );
    }
}
