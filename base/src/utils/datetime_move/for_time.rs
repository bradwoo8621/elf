use crate::{
    DateMoveUtils, DateTimeMovementType, DateTimeMovementUnit, DateTimeMovements, MoveUtilsForTime,
};
use chrono::{NaiveTime, Timelike};

impl DateMoveUtils<NaiveTime> for NaiveTime {
    fn move_to(&self, movements: &DateTimeMovements) -> Option<NaiveTime> {
        let mut current = self.clone();

        for movement in movements.iter() {
            current = match movement.unit {
                DateTimeMovementUnit::Hour => {
                    current.move_to_hour(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Minute => {
                    current.move_to_minute(&movement.r#type, movement.offset)?
                }
                DateTimeMovementUnit::Second => {
                    current.move_to_second(&movement.r#type, movement.offset)?
                }
                // ignore the date part
                _ => current,
            };
        }

        Some(current)
    }
}

impl MoveUtilsForTime for NaiveTime {
    fn move_to_hour(self, r#type: &DateTimeMovementType, offset_or_hour: u32) -> Option<Self> {
        let current_hour = self.hour();
        let hour = match r#type {
            DateTimeMovementType::Plus => (current_hour + offset_or_hour) % 24,
            DateTimeMovementType::Minus => {
                if (offset_or_hour) < current_hour {
                    current_hour - offset_or_hour
                } else {
                    current_hour + 24 - (offset_or_hour % 24)
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_hour > 23 {
                    23
                } else {
                    offset_or_hour
                }
            }
        };
        if hour == current_hour {
            Some(self)
        } else {
            self.with_hour(hour)
        }
    }

    fn move_to_minute(self, r#type: &DateTimeMovementType, offset_or_minute: u32) -> Option<Self> {
        let current_hour = self.hour();
        let current_minute = self.minute();
        let (hour, minute) = match r#type {
            DateTimeMovementType::Plus => {
                let offset_hour = offset_or_minute / 60;
                let offset_minute = offset_or_minute % 60;
                if current_minute + offset_minute > 60 {
                    (
                        (current_hour + offset_hour + 1) % 24,
                        current_minute + offset_minute - 60,
                    )
                } else {
                    (
                        (current_hour + offset_hour) % 24,
                        current_minute + offset_minute,
                    )
                }
            }
            DateTimeMovementType::Minus => {
                let offset_hour = offset_or_minute / 60;
                let offset_minute = offset_or_minute % 60;
                if current_minute > offset_minute {
                    (
                        (current_hour - offset_hour) % 24,
                        current_minute - offset_minute,
                    )
                } else {
                    (
                        (current_hour - offset_hour - 1) % 24,
                        current_minute + 60 - offset_minute,
                    )
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_minute > 59 {
                    (current_hour, 59)
                } else {
                    (current_hour, offset_or_minute)
                }
            }
        };

        match (current_hour == hour, current_minute == minute) {
            (true, true) => Some(self),
            (true, false) => self.with_hour(hour),
            (false, true) => self.with_minute(minute),
            (false, false) => self.with_hour(hour)?.with_minute(minute),
        }
    }

    fn move_to_second(self, r#type: &DateTimeMovementType, offset_or_second: u32) -> Option<Self> {
        let current_hour = self.hour();
        let current_minute = self.minute();
        let current_second = self.second();
        let (hour, minute, second) = match r#type {
            DateTimeMovementType::Plus => {
                let mut offset_hour = offset_or_second / 3600;
                let mut offset_minute = offset_or_second % 3600 / 60;
                let mut offset_second = offset_or_second % 60;
                if current_second + offset_second > 60 {
                    offset_second -= 60;
                    offset_minute += 1
                }
                if current_minute + offset_minute > 60 {
                    offset_minute -= 60;
                    offset_hour += 1;
                }
                (
                    (current_hour + offset_hour) % 24,
                    current_minute + offset_minute,
                    current_second + offset_second,
                )
            }
            DateTimeMovementType::Minus => {
                let mut offset_hour = offset_or_second / 3600;
                let mut offset_minute = offset_or_second % 3600 / 60;
                let mut offset_second = offset_or_second % 60;
                if current_second < offset_second {
                    offset_second -= 60;
                    offset_minute += 1
                }
                if current_minute < offset_minute {
                    offset_minute -= 60;
                    offset_hour += 1;
                }
                if offset_hour > 24 {
                    offset_hour %= 24;
                }
                if current_hour > offset_hour {
                    (
                        current_hour - offset_hour,
                        current_minute - offset_minute,
                        current_second - offset_second,
                    )
                } else {
                    (
                        current_hour - offset_hour,
                        current_minute - offset_minute,
                        current_second - offset_second,
                    )
                }
            }
            DateTimeMovementType::Set => {
                if offset_or_second > 59 {
                    (current_hour, current_minute, 59)
                } else {
                    (current_hour, current_minute, offset_or_second)
                }
            }
        };

        match (
            current_hour == hour,
            current_minute == minute,
            current_second == second,
        ) {
            (true, true, true) => Some(self),
            (true, true, false) => self.with_second(second),
            (true, false, true) => self.with_minute(minute),
            (true, false, false) => self.with_minute(hour)?.with_second(second),
            (false, true, true) => self.with_hour(hour),
            (false, true, false) => self.with_hour(hour)?.with_second(second),
            (false, false, true) => self.with_hour(hour)?.with_minute(minute),
            (false, false, false) => self
                .with_hour(hour)?
                .with_minute(minute)?
                .with_second(second),
        }
    }
}
