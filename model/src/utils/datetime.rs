use crate::StdR;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub trait DateTimeUtils {
    fn to_date(&self) -> StdR<NaiveDate>;
    /// to date, when string can be [to_date] or [to_datetime],
    /// the time part will be discarded
    fn to_date_loose(&self) -> StdR<NaiveDate>;
    fn to_time(&self) -> StdR<NaiveTime>;
    fn to_datetime(&self) -> StdR<NaiveDateTime>;
}

impl DateTimeUtils for String {
    fn to_date(&self) -> StdR<NaiveDate> {
        todo!("implement to_date for String")
    }

    fn to_date_loose(&self) -> StdR<NaiveDate> {
        todo!("implement to_date_loose for String")
    }

    fn to_time(&self) -> StdR<NaiveTime> {
        todo!("implement to_time for String")
    }

    fn to_datetime(&self) -> StdR<NaiveDateTime> {
        todo!("implement to_datetime for String")
    }
}
