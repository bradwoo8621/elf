use crate::{DateTimeMovementType, DateTimeMovements};
use chrono::{Datelike, Timelike};

pub trait MoveUtilsForDate
where
    Self: Datelike + Clone,
{
    fn move_to_year(&self, r#type: &DateTimeMovementType, offset_or_year: u32) -> Option<Self> {
        let year = match r#type {
            DateTimeMovementType::Plus => self.year() + offset_or_year as i32,
            DateTimeMovementType::Minus => self.year() - offset_or_year as i32,
            DateTimeMovementType::Set => offset_or_year as i32,
        };

        if year == self.year() {
            return Some(self.clone());
        }

        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => self.with_year(year),
            2 => {
                if self.day() <= 28 {
                    self.with_year(year)
                } else {
                    // day is 29
                    match year {
                        y if y % 400 == 0 => self.with_year(year),
                        y if y % 100 == 0 => self.with_day(28)?.with_year(year),
                        y if y % 4 == 0 => self.with_year(year),
                        _ => self.with_day(28)?.with_year(year),
                    }
                }
            }
            4 | 6 | 9 | 11 => self.with_year(year),
            // never happen
            _ => self.with_year(year),
        }
    }

    fn move_to_month(&self, r#type: &DateTimeMovementType, offset_or_month: u32) -> Option<Self> {
        let current_year = self.year();
        let current_month = self.month();
        let (year, month) = match r#type {
            DateTimeMovementType::Plus => {
                let offset_year = (offset_or_month / 12) as i32;
                let offset_month = offset_or_month % 12;
                if offset_month + current_month > 12 {
                    (
                        current_year + offset_year + 1,
                        current_month + offset_month - 12,
                    )
                } else {
                    (current_year + offset_year, current_month + offset_month)
                }
            }
            DateTimeMovementType::Minus => {
                let offset_year = (offset_or_month / 12) as i32;
                let offset_month = offset_or_month % 12;
                if current_month > offset_month {
                    (current_year - offset_year, current_month - offset_month)
                } else {
                    (
                        current_year - offset_year - 1,
                        current_month + 12 - offset_month,
                    )
                }
            }
            DateTimeMovementType::Set => (
                current_year,
                if offset_or_month > 12 {
                    12
                } else if offset_or_month < 1 {
                    1
                } else {
                    offset_or_month
                },
            ),
        };

        if current_year == year && current_month == month {
            return Some(self.clone());
        }

        let moved = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => self.with_month(month),
            2 => {
                if self.day() <= 28 {
                    self.with_month(month)
                } else {
                    // day is 29
                    match year {
                        y if y % 400 == 0 => self.with_month(month),
                        y if y % 100 == 0 => self.with_day(28)?.with_month(month),
                        y if y % 4 == 0 => self.with_month(month),
                        _ => self.with_day(28)?.with_month(month),
                    }
                }
            }
            4 | 6 | 9 | 11 => self.with_month(month),
            // never happen
            _ => self.with_month(month),
        };

        if current_year != year {
            moved?.with_year(year)
        } else {
            moved
        }
    }

    fn move_to_day(&self, r#type: &DateTimeMovementType, offset_or_day: u32) -> Option<Self>;
}

pub trait MoveUtilsForTime
where
    Self: Timelike + Clone,
{
    fn move_to_hour(&self, r#type: &DateTimeMovementType, offset_or_hour: u32) -> Option<Self>;
    fn move_to_minute(&self, r#type: &DateTimeMovementType, offset_or_minute: u32) -> Option<Self>;
    fn move_to_second(&self, r#type: &DateTimeMovementType, offset_or_second: u32) -> Option<Self>;
}

pub trait DateMoveUtils<S> {
    fn move_to(&self, movements: &DateTimeMovements) -> Option<S>;
}
