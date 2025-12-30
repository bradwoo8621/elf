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
