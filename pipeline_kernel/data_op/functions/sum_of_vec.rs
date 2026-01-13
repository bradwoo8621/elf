use crate::ArcTopicDataValue;
use bigdecimal::{BigDecimal, Zero};
use elf_base::StdR;
use std::ops::Deref;
use std::sync::Arc;

impl ArcTopicDataValue {
    /// none and empty string are treated as 0
    /// return 0 when there is no elements.
    pub fn sum<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => {
                let mut sum: BigDecimal = BigDecimal::zero();
                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => continue,
                        ArcTopicDataValue::Str(str) => {
                            if !str.is_empty() {
                                let decimal = value.try_to_decimal()?;
                                sum = sum + decimal.deref();
                            }
                        }
                        _ => {
                            let decimal = value.try_to_decimal()?;
                            sum = sum + decimal.deref();
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(sum))))
            }
            _ => not_support(),
        }
    }
}
