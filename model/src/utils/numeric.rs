use crate::{StdErrCode, StdErrorCode, StdR};
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub trait NumericUtils {
    fn is_numeric(&self) -> bool;
    fn to_decimal(&self) -> StdR<BigDecimal>;
}

impl NumericUtils for String {
    /// radix 10
    fn is_numeric(&self) -> bool {
        self.chars().all(|c| c.is_digit(10))
    }

    fn to_decimal(&self) -> StdR<BigDecimal> {
        if let Ok(v) = BigDecimal::from_str(&self) {
            Ok(v)
        } else {
            StdErrCode::DecimalParse.msg(format!("Cannot parse '{}' to decimal", self))
        }
    }
}
