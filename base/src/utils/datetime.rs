use crate::{
    DateFormatter, DateTimeFormatter, DateTimeFormatterBase, ErrorCode, FullDateTimeFormatter,
    LooseDateFormatter, StdErrCode, StdR, TimeFormatter,
};
use bigdecimal::num_bigint::ToBigInt;
use bigdecimal::{BigDecimal, Signed, ToPrimitive};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::fmt::Display;

pub const DEFAULT_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";
pub const DEFAULT_TIME_FORMAT: &str = "%H:%M:%S";

pub trait DateTimeUtils {
    fn to_date(&self) -> StdR<NaiveDate>;
    /// to date, when string can be [to_date] or [to_datetime],
    /// the time part will be discarded
    fn to_date_loose(&self) -> StdR<NaiveDate>;
    fn to_time(&self) -> StdR<NaiveTime>;
    fn to_datetime(&self) -> StdR<NaiveDateTime>;
    fn to_full_datetime(&self) -> StdR<NaiveDateTime>;
    /// to date, when string can be [to_date] or [to_datetime],
    /// the time part will be [00:00:00] if not appears
    fn to_datetime_loose(&self) -> StdR<NaiveDateTime>;
}

impl DateTimeUtils for String {
    fn to_date(&self) -> StdR<NaiveDate> {
        DateFormatter::parse(self)
    }

    fn to_date_loose(&self) -> StdR<NaiveDate> {
        LooseDateFormatter::parse_date(self)
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        TimeFormatter::parse(self)
    }

    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeFormatter::parse(self)
    }

    fn to_full_datetime(&self) -> StdR<NaiveDateTime> {
        FullDateTimeFormatter::parse(self)
    }

    fn to_datetime_loose(&self) -> StdR<NaiveDateTime> {
        LooseDateFormatter::parse_datetime(self)
    }
}

struct DateTimeUtilsBaseForNumber;

impl DateTimeUtilsBaseForNumber {
    #[track_caller]
    fn not_negative<R, V, EC>(value: V, error_code: EC, target_type: &str) -> StdR<R>
    where
        V: Display,
        EC: ErrorCode,
    {
        error_code.msg(format!(
            "Cannot parse the given value[{}] into a {}, negative value is not allowed.",
            value, target_type
        ))
    }

    #[track_caller]
    fn parse_failed<R, V, EC>(value: V, error_code: EC, target_type: &str) -> StdR<R>
    where
        V: Display,
        EC: ErrorCode,
    {
        error_code.msg(format!(
            "Cannot parse the given value[{}] into a {}.",
            value, target_type
        ))
    }

    fn parse_i64<T, ToT, EC>(value: &i64, to_t: ToT, error_code: EC, target_type: &str) -> StdR<T>
    where
        ToT: FnOnce(DateTime<Utc>) -> T,
        EC: ErrorCode,
    {
        match *value {
            x if x < 0 => DateTimeUtilsBaseForNumber::not_negative(x, error_code, target_type),
            x => {
                if let Some(datetime) = DateTime::from_timestamp_millis(x) {
                    Ok(to_t(datetime))
                } else {
                    DateTimeUtilsBaseForNumber::parse_failed(x, error_code, target_type)
                }
            }
        }
    }

    fn parse_u64<T, ToT, EC>(value: &u64, to_t: ToT, error_code: EC, target_type: &str) -> StdR<T>
    where
        ToT: FnOnce(DateTime<Utc>) -> T,
        EC: ErrorCode,
    {
        if let Some(datetime) = DateTime::from_timestamp_millis(*value as i64) {
            Ok(to_t(datetime))
        } else {
            DateTimeUtilsBaseForNumber::parse_failed(value, error_code, target_type)
        }
    }

    fn parse_decimal<T, ToT, EC>(
        value: &BigDecimal,
        to_t: ToT,
        error_code: EC,
        target_type: &str,
    ) -> StdR<T>
    where
        ToT: FnOnce(DateTime<Utc>) -> T,
        EC: ErrorCode,
    {
        let value = value.clone();
        match value {
            x if x.is_negative() => {
                DateTimeUtilsBaseForNumber::not_negative(x, error_code, target_type)
            }
            x => {
                if let Some(xv) = x.to_bigint() {
                    if let Some(xv) = xv.to_i64() {
                        if let Some(datetime) = DateTime::from_timestamp_millis(xv) {
                            return Ok(to_t(datetime));
                        }
                    }
                }
                DateTimeUtilsBaseForNumber::parse_failed(x, error_code, target_type)
            }
        }
    }
}

impl DateTimeUtils for i64 {
    /// return utc date
    fn to_date(&self) -> StdR<NaiveDate> {
        DateTimeUtilsBaseForNumber::parse_i64(
            self,
            |d| d.naive_utc().date(),
            StdErrCode::DateParse,
            "date",
        )
    }

    /// return utc date
    fn to_date_loose(&self) -> StdR<NaiveDate> {
        self.to_date()
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        DateTimeUtilsBaseForNumber::parse_i64(self, |d| d.time(), StdErrCode::TimeParse, "time")
    }

    /// return utc date
    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_i64(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_full_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_i64(
            self,
            |d| d.naive_utc(),
            StdErrCode::FullDateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_datetime_loose(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_i64(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }
}

/// as i64, and transform
impl DateTimeUtils for u64 {
    /// return utc date
    fn to_date(&self) -> StdR<NaiveDate> {
        DateTimeUtilsBaseForNumber::parse_u64(
            self,
            |d| d.naive_utc().date(),
            StdErrCode::DateParse,
            "date",
        )
    }

    /// return utc date
    fn to_date_loose(&self) -> StdR<NaiveDate> {
        self.to_date()
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        DateTimeUtilsBaseForNumber::parse_u64(self, |d| d.time(), StdErrCode::TimeParse, "time")
    }

    /// return utc date
    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_u64(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_full_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_u64(
            self,
            |d| d.naive_utc(),
            StdErrCode::FullDateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_datetime_loose(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_u64(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }
}

/// as i64, and transform
impl DateTimeUtils for BigDecimal {
    /// return utc date
    fn to_date(&self) -> StdR<NaiveDate> {
        DateTimeUtilsBaseForNumber::parse_decimal(
            self,
            |d| d.naive_utc().date(),
            StdErrCode::DateParse,
            "date",
        )
    }

    /// return utc date
    fn to_date_loose(&self) -> StdR<NaiveDate> {
        self.to_date()
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        DateTimeUtilsBaseForNumber::parse_decimal(self, |d| d.time(), StdErrCode::TimeParse, "time")
    }

    /// return utc date
    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_decimal(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_full_datetime(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_decimal(
            self,
            |d| d.naive_utc(),
            StdErrCode::FullDateTimeParse,
            "datetime",
        )
    }

    /// return utc date
    fn to_datetime_loose(&self) -> StdR<NaiveDateTime> {
        DateTimeUtilsBaseForNumber::parse_decimal(
            self,
            |d| d.naive_utc(),
            StdErrCode::DateTimeParse,
            "datetime",
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::DateTimeUtils;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_time() {
        let time = "01:02:03"
            .to_string()
            .to_time()
            .expect("Failed to convert to time.");
        assert_eq!(time.hour(), 1);
        assert_eq!(time.minute(), 2);
        assert_eq!(time.second(), 3);
        assert_eq!(time.nanosecond(), 0);

        let time = "233445"
            .to_string()
            .to_time()
            .expect("Failed to convert to time.");
        assert_eq!(time.hour(), 23);
        assert_eq!(time.minute(), 34);
        assert_eq!(time.second(), 45);
        assert_eq!(time.nanosecond(), 0);
    }

    #[test]
    fn test_date() {
        let date = "2025-12-30"
            .to_string()
            .to_date()
            .expect("Failed to convert to date.");
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 30);

        let date = "2025-12-30 01:02:03"
            .to_string()
            .to_date_loose()
            .expect("Failed to convert to date.");
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 30);
    }

    #[test]
    fn test_datetime() {
        let datetime = "2025-12-30 01:02:03"
            .to_string()
            .to_datetime()
            .expect("Failed to convert to datetime.");
        assert_eq!(datetime.year(), 2025);
        assert_eq!(datetime.month(), 12);
        assert_eq!(datetime.day(), 30);
        assert_eq!(datetime.hour(), 1);
        assert_eq!(datetime.minute(), 2);
        assert_eq!(datetime.second(), 3);
        assert_eq!(datetime.nanosecond(), 0);

        let datetime = "2025-12-30 01:02"
            .to_string()
            .to_datetime()
            .expect("Failed to convert to datetime.");
        assert_eq!(datetime.year(), 2025);
        assert_eq!(datetime.month(), 12);
        assert_eq!(datetime.day(), 30);
        assert_eq!(datetime.hour(), 1);
        assert_eq!(datetime.minute(), 2);
        assert_eq!(datetime.second(), 0);
        assert_eq!(datetime.nanosecond(), 0);

        let datetime = "2025-12-30"
            .to_string()
            .to_datetime_loose()
            .expect("Failed to convert to datetime.");
        assert_eq!(datetime.year(), 2025);
        assert_eq!(datetime.month(), 12);
        assert_eq!(datetime.day(), 30);
        assert_eq!(datetime.hour(), 0);
        assert_eq!(datetime.minute(), 0);
        assert_eq!(datetime.second(), 0);
        assert_eq!(datetime.nanosecond(), 0);
    }
}
