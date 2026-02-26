use crate::{
    ErrorCode, StdErrCode, StdR, DEFAULT_DATETIME_FORMAT, DEFAULT_DATE_FORMAT, DEFAULT_TIME_FORMAT,
};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub trait StringUtils {
    fn is_blank(&self) -> bool;
    fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

impl StringUtils for &str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
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

pub trait StringConverterFrom {
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
        value.format(DEFAULT_DATETIME_FORMAT).to_string()
    }

    fn from_date(value: &NaiveDate) -> String {
        value.format(DEFAULT_DATE_FORMAT).to_string()
    }

    fn from_time(value: &NaiveTime) -> String {
        value.format(DEFAULT_TIME_FORMAT).to_string()
    }
}

impl StringConverterFrom for String {}

pub trait StringConverterTo
where
    Self: AsRef<str>,
{
    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        NaiveDateTime::parse_from_str(self.as_ref(), DEFAULT_DATETIME_FORMAT)
            .map_err(|e| StdErrCode::DateTimeParse.e_msg(format!("{}", e)))
    }

    fn to_date(&self) -> StdR<NaiveDate> {
        NaiveDate::parse_from_str(self.as_ref(), DEFAULT_DATE_FORMAT)
            .map_err(|e| StdErrCode::DateTimeParse.e_msg(format!("{}", e)))
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        NaiveTime::parse_from_str(self.as_ref(), DEFAULT_TIME_FORMAT)
            .map_err(|e| StdErrCode::DateTimeParse.e_msg(format!("{}", e)))
    }
}

impl StringConverterTo for &str {}

impl StringConverterTo for String {}
