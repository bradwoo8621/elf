use crate::{
    DateFormatter, DateTimeFormatter, DateTimeFormatterBase, FullDateTimeFormatter,
    LooseDateFormatter, StdR, TimeFormatter,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

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
