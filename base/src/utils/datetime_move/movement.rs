use crate::{ErrorCode, StdErrCode, StdR, VoidR};
use bigdecimal::{BigDecimal, Signed, ToPrimitive};
use std::str::FromStr;

pub enum DateTimeMovementUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

pub enum DateTimeMovementType {
    Plus,
    Minus,
    Set,
}

pub struct DateTimeMovement {
    pub unit: DateTimeMovementUnit,
    pub r#type: DateTimeMovementType,
    pub offset: u16,
}

impl DateTimeMovement {
    fn create(unit: DateTimeMovementUnit) -> Self {
        Self {
            unit,
            r#type: DateTimeMovementType::Set,
            offset: 0,
        }
    }

    pub fn of(unit: &String, r#type: &BigDecimal, offset: &BigDecimal) -> StdR<Self> {
        let unit = match unit.as_str() {
            "Y" => DateTimeMovementUnit::Year,
            "M" => DateTimeMovementUnit::Month,
            "D" => DateTimeMovementUnit::Day,
            "h" => DateTimeMovementUnit::Hour,
            "m" => DateTimeMovementUnit::Minute,
            "s" => DateTimeMovementUnit::Second,
            v => {
                return StdErrCode::DateMovementParse
                    .msg(format!("Movement unit[{}] not supported.", v));
            }
        };

        let r#type = match r#type {
            v if v > 0 => DateTimeMovementType::Plus,
            v if v < 0 => DateTimeMovementType::Minus,
            _ => DateTimeMovementType::Set,
        };

        let offset = if offset.is_negative() || !offset.is_integer() {
            return StdErrCode::DateMovementParse.msg(format!(
                "Movement offset[{}] must be integer and cannot be negative.",
                offset
            ));
        } else if let Some(value) = offset.to_u16() {
            value
        } else {
            return StdErrCode::DateMovementParse.msg(format!(
                "Movement offset[{}] must between 0 and 2^16.",
                offset
            ));
        };

        Ok(Self {
            unit,
            r#type,
            offset,
        })
    }
}

pub type DateTimeMovements = Vec<DateTimeMovement>;

pub struct DateTimeMoveSupport;

impl DateTimeMoveSupport {
    fn parse_fail<R>(str: &String) -> StdR<R> {
        StdErrCode::DateMovementParse.msg(format!("Cannot parse given movement[{}].", str))
    }

    /// create movement when current movement not declared,
    /// otherwise raise error
    fn create_move(
        current_movement: &Option<DateTimeMovement>,
        unit: DateTimeMovementUnit,
        str: &String,
    ) -> StdR<Option<DateTimeMovement>> {
        if current_movement.is_none() {
            Ok(Some(DateTimeMovement::create(unit)))
        } else {
            Self::parse_fail(str)
        }
    }

    /// set type when movement declared, and type not declared,
    /// otherwise raise error
    fn update_move_type(
        movement: &mut Option<DateTimeMovement>,
        r#type: DateTimeMovementType,
        str: &String,
    ) -> VoidR {
        if let Some(movement) = movement {
            match movement.r#type {
                DateTimeMovementType::Set => {
                    movement.r#type = r#type;
                    Ok(())
                }
                _ => Self::parse_fail(str),
            }
        } else {
            Self::parse_fail(&str)
        }
    }

    pub fn parse(str: &String) -> StdR<DateTimeMovements> {
        let mut movements: DateTimeMovements = vec![];

        let mut current_move = None;
        let mut digits = vec![];

        for char in str.trim().chars() {
            match char {
                'Y' => {
                    current_move =
                        Self::create_move(&current_move, DateTimeMovementUnit::Year, str)?
                }
                'M' => {
                    current_move =
                        Self::create_move(&current_move, DateTimeMovementUnit::Month, str)?
                }
                'D' => {
                    current_move = Self::create_move(&current_move, DateTimeMovementUnit::Day, str)?
                }
                'h' => {
                    current_move =
                        Self::create_move(&current_move, DateTimeMovementUnit::Hour, str)?
                }
                'm' => {
                    current_move =
                        Self::create_move(&current_move, DateTimeMovementUnit::Minute, str)?
                }
                's' => {
                    current_move =
                        Self::create_move(&current_move, DateTimeMovementUnit::Second, str)?
                }
                '+' => Self::update_move_type(&mut current_move, DateTimeMovementType::Plus, str)?,
                '-' => Self::update_move_type(&mut current_move, DateTimeMovementType::Minus, str)?,
                '0'..='9' => {
                    // 0-9 only allowed after unit or type declared
                    if current_move.is_none() {
                        return Self::parse_fail(str);
                    }
                    digits.push(char);
                }
                c if c.is_whitespace() => {
                    if current_move.is_none() || digits.is_empty() {
                        // no movement created, or digits part not start, ignore whitespace
                        continue;
                    }

                    if let Ok(offset) = u16::from_str(digits.iter().collect::<String>().as_str()) {
                        // take over movement, set offset, and push to movements
                        let mut movement = current_move.take().unwrap();
                        movement.offset = offset;
                        movements.push(movement);
                    } else {
                        return Self::parse_fail(str);
                    }
                }
                _ => return Self::parse_fail(str),
            }
        }

        if current_move.is_some() {
            Self::parse_fail(str)
        } else {
            Ok(movements)
        }
    }
}
