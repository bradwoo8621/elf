use crate::{ArcTopicDataValue, MinmaxCompare, MinmaxFinder, MinmaxState};
use elf_base::{StdErr, StdR};
use std::sync::Arc;

pub trait Minmax {
    fn max_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn max_decimal_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn max_date_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn max_datetime_value<NotSupport>(
        self,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn max_time_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_decimal_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_date_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_datetime_value<NotSupport>(
        self,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn min_time_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;
}

impl Minmax for &Arc<Vec<Arc<ArcTopicDataValue>>> {
    fn max_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find(MinmaxState::new(MinmaxCompare::Greater), not_support)
    }

    fn max_decimal_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_decimal(MinmaxState::new(MinmaxCompare::Greater), not_support)
    }

    fn max_date_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_date(MinmaxState::new(MinmaxCompare::Greater), not_support)
    }

    fn max_datetime_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_datetime(MinmaxState::new(MinmaxCompare::Greater), not_support)
    }

    fn max_time_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_time(MinmaxState::new(MinmaxCompare::Greater), not_support)
    }

    fn min_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find(MinmaxState::new(MinmaxCompare::Less), not_support)
    }

    fn min_decimal_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_decimal(MinmaxState::new(MinmaxCompare::Less), not_support)
    }

    fn min_date_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_date(MinmaxState::new(MinmaxCompare::Less), not_support)
    }

    fn min_datetime_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_datetime(MinmaxState::new(MinmaxCompare::Less), not_support)
    }

    fn min_time_value<NotSupport>(self, not_support: NotSupport) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_time(MinmaxState::new(MinmaxCompare::Less), not_support)
    }
}

#[cfg(test)]
mod tests {
	use crate::{ArcTopicDataValue, Minmax, MinmaxCompare, MinmaxState, PipelineKernelErrorCode};
	use bigdecimal::BigDecimal;
	use chrono::{NaiveDate, NaiveDateTime};
	use elf_base::ErrorCode;
	use std::ops::Deref;
	use std::str::FromStr;
	use std::sync::Arc;

	#[test]
    fn test() {
        let mut minmax = MinmaxState::new(MinmaxCompare::Less);
        let not_support = || PipelineKernelErrorCode::VariableFuncNotSupported.e();
        minmax
            .exchange_with_decimal(
                &Arc::new(BigDecimal::from_str("100").unwrap()),
                &not_support,
            )
            .expect("100 not supported");
        minmax
            .exchange_with_decimal(
                &Arc::new(BigDecimal::from_str("200").unwrap()),
                &not_support,
            )
            .expect("200 not supported");
        println!("{:?}", minmax);
    }

    #[test]
    fn test_all_decimal() {
        let vec: Vec<Arc<ArcTopicDataValue>> = vec![
            ArcTopicDataValue::Num(Arc::new(BigDecimal::from_str("100").unwrap())),
            ArcTopicDataValue::Num(Arc::new(BigDecimal::from_str("200").unwrap())),
            ArcTopicDataValue::Str(Arc::new("50".to_string())),
        ]
        .into_iter()
        .map(Arc::new)
        .collect();
        let vec = &Arc::new(vec);
        let min = vec.min_value(|| PipelineKernelErrorCode::VariableFuncNotSupported.e());
        assert!(min.is_ok());
        let min = min.unwrap();
        assert!(matches!(min.deref(), &ArcTopicDataValue::Num(_)));
        match min.deref() {
            ArcTopicDataValue::Num(v) => {
                assert_eq!(v.to_plain_string(), "50")
            }
            _ => panic!("wrong"),
        }

        println!("{:?}", vec[2]);
    }

    #[test]
    fn test_date_and_datetime() {
        let vec: Vec<Arc<ArcTopicDataValue>> = vec![
            ArcTopicDataValue::Date(Arc::new(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap())),
            ArcTopicDataValue::DateTime(Arc::new(
                NaiveDateTime::parse_from_str("2024-01-02 01:02:03", "%Y-%m-%d %H:%M:%S").unwrap(),
            )),
        ]
        .into_iter()
        .map(Arc::new)
        .collect();
        let vec = &Arc::new(vec);
        let min = vec.min_value(|| PipelineKernelErrorCode::VariableFuncNotSupported.e());
        assert!(min.is_ok());
        let min = min.unwrap();
        assert!(matches!(min.deref(), &ArcTopicDataValue::Date(_)));
        match min.deref() {
            ArcTopicDataValue::Date(v) => {
                assert_eq!(v.format("%Y-%m-%d").to_string().as_str(), "2024-01-02")
            }
            _ => panic!("wrong"),
        }

        println!("{:?}", vec);
    }
}
