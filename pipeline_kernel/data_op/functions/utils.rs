use crate::ArcTopicDataValue;
use bigdecimal::BigDecimal;
use elf_base::StdR;
use std::sync::Arc;

/// utilities
impl ArcTopicDataValue {
    pub fn value_as_num<DecimalParseErr>(
        value: Option<BigDecimal>,
        decimal_parse_err: DecimalParseErr,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        // decimal parse error
        DecimalParseErr: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        value
            .map(|value| Ok(Arc::new(Self::Num(Arc::new(value)))))
            .unwrap_or(decimal_parse_err())
    }
}
