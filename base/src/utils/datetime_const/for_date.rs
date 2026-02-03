use crate::{
    HALF_YEAR_FIRST, HALF_YEAR_SECOND, QUARTER_FIRST, QUARTER_FOURTH, QUARTER_SECOND,
    QUARTER_THIRD, TEN_DAYS_FIRST, TEN_DAYS_SECOND, TEN_DAYS_THIRD,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};

pub trait DateConstValues
where
    Self: Datelike,
{
    fn half_year(&self) -> u8 {
        if self.month() <= 6 {
            HALF_YEAR_FIRST
        } else {
            HALF_YEAR_SECOND
        }
    }

    fn quarter(&self) -> u8 {
        let month = self.month();
        if month <= 3 {
            QUARTER_FIRST
        } else if month <= 6 {
            QUARTER_SECOND
        } else if month <= 9 {
            QUARTER_THIRD
        } else {
            QUARTER_FOURTH
        }
    }

    fn ten_days(&self) -> u8 {
        let day = self.day();
        if day <= 10 {
            TEN_DAYS_FIRST
        } else if day <= 20 {
            TEN_DAYS_SECOND
        } else {
            TEN_DAYS_THIRD
        }
    }

    /// - if 1st Jan. of current year is Sunday, week starts from 1.
    /// - if 1st Jan. of current year is not Sunday, week starts from 0.
    /// - max week is 53.
    fn week_of_year(&self) -> u8 {
        let jan1 = NaiveDate::from_ymd_opt(self.year(), 1, 1).unwrap();
        // sun: 0, mon: 1, ... sat: 6
        // if value is not zero, means there is a week zero, and has (7 - value) days in week zero
        let jan1_weekday = jan1.weekday().num_days_from_sunday() as i32;
        // days between 1st Jan. and current day. include 1st Jan., exclude current day
        let days_since_jan1 = self.num_days_from_ce() - jan1.num_days_from_ce();
        match jan1_weekday {
            // there is no week 0, 1st Jan. is Sunday
            0 => ((days_since_jan1 + 7) / 7) as u8,
            // there is week 0, and week 0 has (7 - jan1_weekday) days
            _ => ((days_since_jan1 + jan1_weekday) / 7) as u8,
        }
    }

    fn week_of_month(&self) -> u8 {
        let first_day = NaiveDate::from_ymd_opt(self.year(), self.month(), 1).unwrap();
        // sun: 0, mon: 1, ... sat: 6
        // if value is not zero, means there is a week zero, and has (7 - value) days in week zero
        let first_day_weekday = first_day.weekday().num_days_from_sunday() as i32;
        // days between first day of month and current day. include first day of month, exclude current day
        let days_since_first_day = self.num_days_from_ce() - first_day.num_days_from_ce();
        match first_day_weekday {
            // there is no week 0, first day of month is Sunday
            0 => ((days_since_first_day + 7) / 7) as u8,
            // there is week 0, and week 0 has (7 - first_day_weekday) days
            _ => ((days_since_first_day + first_day_weekday) / 7) as u8,
        }
    }
}

impl DateConstValues for NaiveDate {}

impl DateConstValues for NaiveDateTime {}

#[cfg(test)]
mod tests {
    use crate::DateConstValues;
    use chrono::{Datelike, NaiveDate};
    use std::str::FromStr;

    #[test]
    fn test_week_of_year() {
        for year in 1900..=2099 {
            for month in 1..=12 {
                let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                let days_of_month = date.num_days_in_month();
                for day in 1..=days_of_month {
                    let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();
                    let formatted = u8::from_str(date.format("%U").to_string().as_str());
                    if let Ok(value) = formatted {
                        assert_eq!(date.week_of_year(), value, "{}", date);
                    } else {
                        panic!("{:?}", date.format("%U").to_string());
                    }
                }
            }
        }
    }
}
