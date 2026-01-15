use crate::ArcTopicDataValue;
use bigdecimal::ToPrimitive;
use elf_base::{NumericUtils, StringUtils};

impl ArcTopicDataValue {
    pub fn try_to_usize(&self) -> Result<usize, &Self> {
        match self {
            Self::Num(decimal) => {
                if let Some(v) = decimal.to_usize() {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            Self::None => Err(self),
            Self::Str(str) => {
                if str.is_blank() {
                    Err(self)
                } else if let Ok(decimal) = str.to_decimal() {
                    if let Some(v) = decimal.to_usize() {
                        Ok(v)
                    } else {
                        Err(self)
                    }
                } else {
                    Err(self)
                }
            }
            _ => Err(self),
        }
    }

    pub fn try_to_usize_or_if_none(&self, none_value: usize) -> Result<usize, &Self> {
        match self {
            Self::None => Ok(none_value),
            _ => self.try_to_usize(),
        }
    }
}
