use crate::{ArcTopicDataValue, MinmaxState};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{StdErr, StdR, StringUtils};

trait MinmaxFinderBase {
    fn with_string_value<NotSupport>(
        minmax: &mut MinmaxState,
        str: &Arc<String>,
        not_support: &NotSupport,
    ) -> Option<StdR<Arc<ArcTopicDataValue>>>
    where
        NotSupport: Fn() -> StdErr;

    fn with_none_and_string_value<NotSupport>(
        value: &Arc<ArcTopicDataValue>,
        minmax: &mut MinmaxState,
        not_support: &NotSupport,
    ) -> Option<StdR<Arc<ArcTopicDataValue>>>
    where
        NotSupport: Fn() -> StdErr,
    {
        match value.deref() {
            ArcTopicDataValue::Str(str) => Self::with_string_value(minmax, str, &not_support),
            _ => None,
        }
    }

    fn find_value<NotSupport, HandleTyped, HandleStringElements>(
        self,
        minmax: MinmaxState,
        typed: HandleTyped,
        string_elements: HandleStringElements,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
        HandleTyped: FnMut(&Arc<ArcTopicDataValue>, &mut MinmaxState, &NotSupport) -> StdR<bool>,
        HandleStringElements: FnOnce(&mut MinmaxState, &NotSupport) -> StdR<bool>;
}

impl MinmaxFinderBase for &Arc<Vec<Arc<ArcTopicDataValue>>> {
    fn with_string_value<NotSupport>(
        minmax: &mut MinmaxState,
        str: &Arc<String>,
        not_support: &NotSupport,
    ) -> Option<StdR<Arc<ArcTopicDataValue>>>
    where
        NotSupport: Fn() -> StdErr,
    {
        if str.is_empty() {
            // ignore empty string
            None
        } else if str.is_blank() {
            // obviously that blank string cannot be cast to any comparable type
            // error
            Some(Err(not_support()))
        } else {
            // postpone, detect types first
            minmax.string_elements.push(str.clone());
            None
        }
    }

    fn find_value<NotSupport, HandleTyped, HandleStringElements>(
        self,
        mut minmax: MinmaxState,
        mut typed: HandleTyped,
        string_elements: HandleStringElements,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
        HandleTyped: FnMut(&Arc<ArcTopicDataValue>, &mut MinmaxState, &NotSupport) -> StdR<bool>,
        HandleStringElements: FnOnce(&mut MinmaxState, &NotSupport) -> StdR<bool>,
    {
        self.is_empty()
            .then_some(Ok(Arc::new(ArcTopicDataValue::None)))
            .unwrap_or_else(|| {
                for value in self.iter() {
                    if let Some(ret) =
                        Self::with_none_and_string_value(value, &mut minmax, &not_support)
                    {
                        return ret;
                    }
                    typed(value, &mut minmax, &not_support)?;
                }

                string_elements(&mut minmax, &not_support)?;
                minmax.get_result()
            })
    }
}

/// - Supports multiple formats: decimal/datetime/date/time.
/// - No element is considered returning `none`,
/// - When calculating the minimum value, `none` and empty string are considered the minimum value `none`.
/// - When calculating the maximum value, `none` and empty string are always ignored.
/// - Blank string always leads an error,
/// - Only the extreme value of a single type will be calculated. If the data contains multiple types, an error will be reported.
///   Unless it only contains both `datetime` and `date` simultaneously.
///   In this case, the `datetime` data will be downgraded to `date`, and the calculated extreme value will also be of the `date` type.
/// - String values will be automatically converted to the detected type. If the conversion fails, an error will be reported.
/// - If there are no explicit values of the four types and `none`,
///   but there are string values, the conversion will be carried out in the priority order of
///   decimal > datetime > date > time, and the compatibility rules are the same as above.
///   Note that once `date` is detected, the already detected `datetime` will also be downgraded.
pub trait MinmaxFinder {
    fn find<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn find_decimal<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn find_date<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn find_datetime<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;

    fn find_time<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr;
}

impl MinmaxFinder for &Arc<Vec<Arc<ArcTopicDataValue>>> {
    fn find<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_value(
            minmax,
            |value, minmax, not_support| match value.deref() {
                ArcTopicDataValue::Num(decimal) => {
                    minmax.exchange_with_decimal(decimal, not_support)
                }
                ArcTopicDataValue::DateTime(datetime) => {
                    minmax.exchange_with_datetime(datetime, not_support)
                }
                ArcTopicDataValue::Date(date) => minmax.exchange_with_date(date, not_support),
                ArcTopicDataValue::Time(time) => minmax.exchange_with_time(time, not_support),
                _ => Err(not_support()),
            },
            |minmax, not_support| minmax.with_string_elements(not_support),
            not_support,
        )
    }

    fn find_decimal<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_value(
            minmax,
            |value, minmax, not_support| match value.deref() {
                ArcTopicDataValue::Num(decimal) => {
                    minmax.exchange_with_decimal(decimal, not_support)
                }
                _ => Err(not_support()),
            },
            |minmax, not_support| minmax.with_decimal_by_string_elements(not_support),
            not_support,
        )
    }

    fn find_date<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_value(
            minmax,
            |value, minmax, not_support| match value.deref() {
                ArcTopicDataValue::Date(date) => minmax.exchange_with_date(date, not_support),
                _ => Err(not_support()),
            },
            |minmax, not_support| minmax.with_date_by_string_elements(not_support),
            not_support,
        )
    }

    fn find_datetime<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_value(
            minmax,
            |value, minmax, not_support| match value.deref() {
                ArcTopicDataValue::DateTime(datetime) => {
                    minmax.exchange_with_datetime(datetime, not_support)
                }
                ArcTopicDataValue::Date(date) => minmax.exchange_with_date(date, not_support),
                _ => Err(not_support()),
            },
            |minmax, not_support| {
                if minmax.has_date {
                    minmax.with_date_by_string_elements(not_support)
                } else {
                    minmax.with_datetime_by_string_elements(not_support)
                }
            },
            not_support,
        )
    }

    fn find_time<NotSupport>(
        self,
        minmax: MinmaxState,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: Fn() -> StdErr,
    {
        self.find_value(
            minmax,
            |value, minmax, not_support| match value.deref() {
                ArcTopicDataValue::Time(time) => minmax.exchange_with_time(time, not_support),
                _ => Err(not_support()),
            },
            |minmax, not_support| minmax.with_time_by_string_elements(not_support),
            not_support,
        )
    }
}
