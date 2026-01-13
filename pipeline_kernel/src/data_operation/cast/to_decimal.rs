use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use elf_base::{NumericUtils, StringUtils};
use std::sync::Arc;

impl ArcTopicDataValue {
    pub fn try_to_decimal(&self) -> Result<Arc<BigDecimal>, &Self> {
        match self {
            Self::Num(decimal) => Ok(decimal.clone()),
            Self::None => Err(self),
            Self::Str(str) => {
                if str.is_blank() {
                    Err(self)
                } else if let Ok(decimal) = str.to_decimal() {
                    Ok(Arc::new(decimal))
                } else {
                    Err(self)
                }
            }
            _ => Err(self),
        }
    }
}
