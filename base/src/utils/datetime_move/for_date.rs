use crate::{
    DateMoveUtils, DateTimeMovementType, DateTimeMovementUnit, DateTimeMovements, MoveUtilsForDate
    ,
};
use chrono::{Datelike, Duration, NaiveDate};

impl DateMoveUtils<NaiveDate> for NaiveDate {
    fn move_to(&self, movements: &DateTimeMovements) -> Option<NaiveDate> {
        let mut current = self.clone();

        for movement in movements.iter() {
            current = match movement.unit {
                DateTimeMovementUnit::Year => {
                    self.move_to_year(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Month => {
                    self.move_to_month(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Day => self.move_to_day(&movement.r#type, movement.offset)?,
                // ignore the time part
                _ => current,
            };
        }

        Some(current)
    }
}

impl MoveUtilsForDate for NaiveDate {
    // noinspection DuplicatedCode
    fn move_to_day(&self, r#type: &DateTimeMovementType, offset_or_day: u16) -> Option<NaiveDate> {
        match r#type {
            DateTimeMovementType::Plus => {
                if offset_or_day == 0 {
                    Some(self.clone())
                } else {
                    self.checked_add_signed(Duration::days(offset_or_day as i64))
                }
            }
            DateTimeMovementType::Minus => {
                if offset_or_day == 0 {
                    Some(self.clone())
                } else {
                    self.checked_sub_signed(Duration::days(offset_or_day as i64))
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_day as u32 == self.day() {
                    Some(self.clone())
                } else if offset_or_day < 1 {
                    self.with_day(1)
                } else {
                    let days_of_month = self.num_days_in_month() as u16;
                    if offset_or_day > days_of_month {
                        self.with_day(days_of_month as u32)
                    } else {
                        self.with_day(offset_or_day as u32)
                    }
                }
            }
        }
    }
}
