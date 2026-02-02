use crate::{
    DateMoveUtils, DateTimeMovementType, DateTimeMovementUnit, DateTimeMovements, MoveUtilsForDate,
    MoveUtilsForTime,
};
use chrono::{Datelike, Duration, NaiveDateTime, Timelike};

impl DateMoveUtils<NaiveDateTime> for NaiveDateTime {
    fn move_to(&self, movements: &DateTimeMovements) -> Option<NaiveDateTime> {
        let mut current = self.clone();

        for movement in movements.iter() {
            current = match movement.unit {
                DateTimeMovementUnit::Year => {
                    current.move_to_year(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Month => {
                    current.move_to_month(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Day => current.move_to_day(&movement.r#type, movement.offset)?,
                DateTimeMovementUnit::Hour => {
                    current.move_to_hour(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Minute => {
                    current.move_to_minute(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Second => {
                    current.move_to_second(&movement.r#type, movement.offset)?
                }
            };
        }

        Some(current)
    }
}

impl MoveUtilsForDate for NaiveDateTime {
    // noinspection DuplicatedCode
    fn move_to_day(
        &self,
        r#type: &DateTimeMovementType,
        offset_or_day: u32,
    ) -> Option<NaiveDateTime> {
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
                if offset_or_day == self.day() {
                    Some(self.clone())
                } else if offset_or_day < 1 {
                    self.with_day(1)
                } else {
                    let days_of_month = self.num_days_in_month() as u16;
                    if offset_or_day > days_of_month as u32 {
                        self.with_day(days_of_month as u32)
                    } else {
                        self.with_day(offset_or_day)
                    }
                }
            }
        }
    }
}

impl MoveUtilsForTime for NaiveDateTime {
    fn move_to_hour(&self, r#type: &DateTimeMovementType, offset_or_hour: u32) -> Option<Self> {
        match r#type {
            DateTimeMovementType::Plus => {
                if offset_or_hour == 0 {
                    Some(self.clone())
                } else {
                    self.checked_add_signed(Duration::hours(offset_or_hour as i64))
                }
            }
            DateTimeMovementType::Minus => {
                if offset_or_hour == 0 {
                    Some(self.clone())
                } else {
                    self.checked_sub_signed(Duration::hours(offset_or_hour as i64))
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_hour == self.hour() {
                    Some(self.clone())
                } else if offset_or_hour > 23 {
                    self.with_hour(23)
                } else {
                    self.with_hour(offset_or_hour)
                }
            }
        }
    }

    fn move_to_minute(&self, r#type: &DateTimeMovementType, offset_or_minute: u32) -> Option<Self> {
        match r#type {
            DateTimeMovementType::Plus => {
                if offset_or_minute == 0 {
                    Some(self.clone())
                } else {
                    self.checked_add_signed(Duration::minutes(offset_or_minute as i64))
                }
            }
            DateTimeMovementType::Minus => {
                if offset_or_minute == 0 {
                    Some(self.clone())
                } else {
                    self.checked_sub_signed(Duration::minutes(offset_or_minute as i64))
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_minute == self.minute() {
                    Some(self.clone())
                } else if offset_or_minute > 59 {
                    self.with_minute(59)
                } else {
                    self.with_minute(offset_or_minute)
                }
            }
        }
    }

    fn move_to_second(&self, r#type: &DateTimeMovementType, offset_or_second: u32) -> Option<Self> {
        match r#type {
            DateTimeMovementType::Plus => {
                if offset_or_second == 0 {
                    Some(self.clone())
                } else {
                    self.checked_add_signed(Duration::seconds(offset_or_second as i64))
                }
            }
            DateTimeMovementType::Minus => {
                if offset_or_second == 0 {
                    Some(self.clone())
                } else {
                    self.checked_sub_signed(Duration::seconds(offset_or_second as i64))
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_second == self.second() {
                    Some(self.clone())
                } else if offset_or_second > 59 {
                    self.with_second(59)
                } else {
                    self.with_second(offset_or_second)
                }
            }
        }
    }
}
