use crate::DateDiffUtils;
use chrono::{Datelike, NaiveDate};

impl DateDiffUtils for NaiveDate {
    fn days_diff(&self, end_date: &NaiveDate) -> i64 {
        (*end_date - *self).num_days()
    }

    fn month_diff(&self, end_date: &NaiveDate) -> i64 {
        let end_year = end_date.year() as i64;
        let end_month = end_date.month() as i64;
        let end_day = end_date.day() as i64;
        let start_year = self.year() as i64;
        let start_month = self.month() as i64;
        let start_day = self.day() as i64;

        if end_year == start_year {
            if end_month == start_month {
                // same year, same month, always return 0
                0
            } else if end_month > start_month {
                if end_day >= start_day {
                    end_month - start_month
                } else {
                    let last_day_of_end_month = end_date.num_days_in_month() as i64;
                    if last_day_of_end_month == end_day && start_day >= end_day {
                        // it is last day of end month
                        end_month - start_month
                    } else {
                        end_month - start_month - 1
                    }
                }
            } else {
                // end date is before start date
                if end_day > start_day {
                    let last_day_of_start_month = self.num_days_in_month() as i64;
                    if last_day_of_start_month == start_day && end_day >= start_day {
                        // it is last day of start month
                        end_month - start_month
                    } else {
                        end_month - start_month + 1
                    }
                } else {
                    end_month - start_month
                }
            }
        } else if end_year > start_year {
            if end_day >= start_day {
                (end_year - start_year) * 12 + end_month - start_month
            } else {
                let last_day_of_end_month = end_date.num_days_in_month() as i64;
                if last_day_of_end_month == end_day && start_day >= end_day {
                    (end_year - start_year) * 12 + end_month - start_month
                } else {
                    (end_year - start_year) * 12 + end_month - start_month + 1
                }
            }
        } else {
            // end year is before start year
            if end_day > start_day {
                let last_day_of_start_month = self.num_days_in_month() as i64;
                if last_day_of_start_month == start_day && end_day >= start_day {
                    // it is last day of start month
                    (end_year - start_year + 1) * 12 + 12 - end_month + start_month
                } else {
                    (end_year - start_year + 1) * 12 + 12 - end_month + start_month - 1
                }
            } else {
                (end_year - start_year + 1) * 12 + 12 - end_month + start_month
            }
        }
    }

    fn year_diff(&self, end_date: &NaiveDate) -> i64 {
        let end_year = end_date.year() as i64;
        let end_month = end_date.month() as i64;
        let end_day = end_date.day() as i64;
        let start_year = self.year() as i64;
        let start_month = self.month() as i64;
        let start_day = self.day() as i64;
        if end_year == start_year {
            // same year, always return 0
            0
        } else if end_year > start_year {
            if end_month == start_month {
                if end_day >= start_day {
                    end_year - start_year
                } else if end_month == 2 {
                    let last_day_of_end_month = end_date.num_days_in_month() as i64;
                    if end_day == last_day_of_end_month {
                        end_year - start_year
                    } else {
                        end_year - start_year - 1
                    }
                } else {
                    end_year - start_year - 1
                }
            } else if end_month > start_month {
                end_year - start_year
            } else {
                end_year - start_year - 1
            }
        } else {
            if end_month == start_month {
                if end_day > start_day {
                    if end_month == 2 {
                        let last_day_of_start_month = self.num_days_in_month() as i64;
                        if start_day == last_day_of_start_month {
                            end_year - start_year
                        } else {
                            end_year - start_year + 1
                        }
                    } else {
                        end_year - start_year + 1
                    }
                } else {
                    end_year - start_year
                }
            } else if end_month > start_month {
                end_year - start_year + 1
            } else {
                end_year - start_year
            }
        }
    }
}
