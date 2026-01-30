use chrono::NaiveDate;

pub trait DateDiffUtils {
    fn days_diff(&self, end_date: &NaiveDate) -> i64;
    fn month_diff(&self, end_date: &NaiveDate) -> i64;
    fn year_diff(&self, end_date: &NaiveDate) -> i64;
}
