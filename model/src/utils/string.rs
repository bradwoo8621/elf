use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub trait StringUtils {
    fn is_blank(&self) -> bool;
    fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

impl StringUtils for Option<String> {
    fn is_blank(&self) -> bool {
        match self {
            Some(s) => s.trim().is_empty(),
            None => true,
        }
    }
}

impl StringUtils for String {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
}

pub trait StringConverter {
    fn from_bool(value: &bool) -> String {
        if *value {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn from_decimal(value: &BigDecimal) -> String {
        value.to_plain_string()
    }

    fn from_datetime(value: &NaiveDateTime) -> String {
        value.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn from_date(value: &NaiveDate) -> String {
        value.format("%Y-%m-%d").to_string()
    }

    fn from_time(value: &NaiveTime) -> String {
        value.format("%H:%M:%S").to_string()
    }
}

impl StringConverter for String {}
