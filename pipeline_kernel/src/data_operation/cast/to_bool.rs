use crate::ArcTopicDataValue;
use bigdecimal::{One, Zero};

impl ArcTopicDataValue {
    /// try to cast itself to bool
    /// boolean -> bool
    /// string [1, true, t, yes, y] -> true
    /// string [0, false, f, no, n] -> false
    /// decimal [1] -> true
    /// decimal [0] -> false
    /// others -> cannot to bool, returns self
    pub fn try_to_bool(&self) -> Result<bool, &Self> {
        match self {
            Self::Bool(b) => Ok(*b),
            Self::Str(s) => match s.to_ascii_lowercase().as_str() {
                "1" | "true" | "t" | "yes" | "y" => Ok(true),
                "0" | "false" | "f" | "no" | "n" => Ok(false),
                _ => Err(self),
            },
            Self::Num(n) => {
                if n.is_one() {
                    Ok(true)
                } else if n.is_zero() {
                    Ok(false)
                } else {
                    Err(self)
                }
            }
            _ => Err(self),
        }
    }
}
