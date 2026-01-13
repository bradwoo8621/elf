use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

impl ArcTopicDataValue {
    /// check itself is string or not
    pub fn is_str(&self) -> Result<&String, &Self> {
        match self {
            Self::Str(s) => Ok(s),
            _ => Err(self),
        }
    }

    /// check itself is bool or not
    pub fn is_bool(&self) -> Result<&bool, &Self> {
        match self {
            Self::Bool(b) => Ok(b),
            _ => Err(self),
        }
    }

    /// check itself is decimal or not
    pub fn is_num(&self) -> Result<&BigDecimal, &Self> {
        match self {
            Self::Num(n) => Ok(n),
            _ => Err(self),
        }
    }

    /// check itself is datetime or not
    pub fn is_datetime(&self) -> Result<&NaiveDateTime, &Self> {
        match self {
            Self::DateTime(dt) => Ok(dt),
            _ => Err(self),
        }
    }

    /// check itself is date or not
    pub fn is_date(&self) -> Result<&NaiveDate, &Self> {
        match self {
            Self::Date(d) => Ok(d),
            _ => Err(self),
        }
    }

    /// check itself is time or not
    pub fn is_time(&self) -> Result<&NaiveTime, &Self> {
        match self {
            Self::Time(t) => Ok(t),
            _ => Err(self),
        }
    }

    /// check itself is date/time/datetime or not
    pub fn is_datetime_related(&self) -> bool {
        match self {
            Self::Date(_) => true,
            Self::DateTime(_) => true,
            Self::Time(_) => true,
            _ => false,
        }
    }
}
