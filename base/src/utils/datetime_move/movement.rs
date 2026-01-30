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
    pub offset: u32,
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
        } else if let Some(value) = offset.to_u32() {
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

pub struct DateTimeMoveSupport<'a> {
    str: &'a str,
    current_move: Option<DateTimeMovement>,
    digits: Vec<char>,

    // results
    movements: DateTimeMovements,
}

impl DateTimeMoveSupport<'_> {
    fn parse_fail<R>(&self) -> StdR<R> {
        StdErrCode::DateMovementParse.msg(format!("Cannot parse given movement[{}].", self.str))
    }

    /// create movement when current movement not declared,
    /// otherwise raise error
    fn create_move(&mut self, unit: DateTimeMovementUnit) -> VoidR {
        if self.current_move.is_none() {
            self.current_move = Some(DateTimeMovement::create(unit));
            Ok(())
        } else {
            self.parse_fail()?
        }
    }

    /// set type when movement declared, and type not declared,
    /// otherwise raise error
    fn update_current_move_type(&mut self, r#type: DateTimeMovementType) -> VoidR {
        if let Some(movement) = &mut self.current_move {
            match movement.r#type {
                DateTimeMovementType::Set => {
                    movement.r#type = r#type;
                    Ok(())
                }
                _ => self.parse_fail(),
            }
        } else {
            self.parse_fail()
        }
    }

    /// try to update movement offset
    /// - has current movement, no offset: parse fail
    /// - has current movement, has offset,
    ///   - offset parse to u16, set offset, clear offset digits,
    ///   - offset cannot parse to u16, parse fail,
    /// - no current movement, no offset: do nothing,
    /// - no current movement, has offset: parse fail.
    fn try_update_move_offset(&mut self) -> VoidR {
        match (&self.current_move, self.digits.is_empty()) {
            (Some(_), true) => self.parse_fail(),
            (Some(_), false) => {
                if let Ok(offset) = u32::from_str(self.digits.iter().collect::<String>().as_str()) {
                    self.digits.clear();
                    // take over current movement
                    let mut movement = self.current_move.take().unwrap();
                    movement.offset = offset;
                    self.movements.push(movement);
                    Ok(())
                } else {
                    self.parse_fail()
                }
            }
            (None, true) => Ok(()),
            (None, false) => self.parse_fail(),
        }
    }

    fn with_unit_detected(&mut self, unit: DateTimeMovementUnit) -> VoidR {
        self.try_update_move_offset()?;
        self.create_move(unit)
    }

    /// - The string must follow the format [Move1Move2...MoveN], where each Move has the format [Unit\[Type\]Offset],
    /// - Unit is one of Y, M, D, h, m, s, representing year, month, day, hour, minute, second respectively,
    /// - Type is one of + or -, representing addition or subtraction.
    ///   If Type is omitted, it indicates direct setting.
    /// - Offset is a number,
    ///   - If it's an addition or subtraction, it must be a positive integer
    ///   - If it's a setting, the valid range varies depending on the Unit:
    ///     - Year: 4 digits,
    ///     - Month: 1 - 12, any value greater than 12, treated as 12,
    ///     - Day: 1 - days of month, any value greater than days of month, treated as last day of month,
    ///     - Hour: 0 - 23, any value greater than 23, treated as 23,
    ///     - Minute: 0 - 59, any value greater than 59, treated as 59,
    ///     - Second: 0 - 59, any value greater than 59, treated as 59,
    /// - whitespaces between moves are allowed, and will be ignored,
    /// - whitespaces between unit and type are allowed, and will be ignored,
    /// - whitespaces between type and offset are allowed, and will be ignored
    fn do_parse(mut self) -> StdR<DateTimeMovements> {
        for char in self.str.trim().chars() {
            match char {
                'Y' => self.with_unit_detected(DateTimeMovementUnit::Year)?,
                'M' => self.with_unit_detected(DateTimeMovementUnit::Month)?,
                'D' => self.with_unit_detected(DateTimeMovementUnit::Day)?,
                'h' => self.with_unit_detected(DateTimeMovementUnit::Hour)?,
                'm' => self.with_unit_detected(DateTimeMovementUnit::Minute)?,
                's' => self.with_unit_detected(DateTimeMovementUnit::Second)?,
                '+' => self.update_current_move_type(DateTimeMovementType::Plus)?,
                '-' => self.update_current_move_type(DateTimeMovementType::Minus)?,
                '0'..='9' => {
                    // 0-9 only allowed after unit or type declared
                    if self.current_move.is_none() {
                        return self.parse_fail();
                    }
                    self.digits.push(char);
                }
                c if c.is_whitespace() => {
                    if self.current_move.is_some() && !self.digits.is_empty() {
                        self.try_update_move_offset()?;
                    }
                }
                _ => return self.parse_fail(),
            }
        }
        if self.current_move.is_some() {
            self.try_update_move_offset()?;
        }
        Ok(self.movements)
    }
}

impl DateTimeMoveSupport<'_> {
    pub fn parse(str: &String) -> StdR<DateTimeMovements> {
        DateTimeMoveSupport {
            str: str.trim(),

            current_move: None,
            digits: vec![],
            movements: vec![],
        }
        .do_parse()
    }
}
