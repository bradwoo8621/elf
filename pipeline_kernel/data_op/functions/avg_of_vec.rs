use std::ops::Deref;
use crate::ArcTopicDataValue;
use bigdecimal::{BigDecimal, Zero};
use elf_base::StdR;
use std::sync::Arc;

impl ArcTopicDataValue {
    /// none and empty string are treated as 0, not count
    /// return 0 when there is no elements.
    pub fn avg<NotSupport>(&self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    return Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(
                        BigDecimal::zero(),
                    ))));
                }

                let mut sum: BigDecimal = BigDecimal::zero();
                let mut count = 0;

                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => continue,
                        ArcTopicDataValue::Str(str) => {
                            if !str.is_empty() {
                                let decimal = value.try_to_decimal()?;
                                sum = sum + decimal.deref();
                                count = count + 1;
                            }
                        }
                        _ => {
                            let decimal = value.try_to_decimal()?;
                            sum = sum + decimal.deref();
                            count = count + 1;
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Num(Arc::new(sum / count))))
            }
            _ => not_support(),
        }
    }
}
