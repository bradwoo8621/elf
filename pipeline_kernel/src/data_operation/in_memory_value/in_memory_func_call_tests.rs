#[cfg(test)]
mod tests {
    use crate::{ArcTopicDataValue, FuncDataPath, InMemoryFuncCall, PathStr};
    use bigdecimal::BigDecimal;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use elf_model::VariablePredefineFunctions;
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;

    fn create_test_path(func: VariablePredefineFunctions) -> FuncDataPath {
        FuncDataPath::new(PathStr::of_str("test.path"), func, Some(vec![]))
    }

    fn create_decimal(value: &str) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Num(Arc::new(
            BigDecimal::from_str(value).unwrap(),
        )))
    }

    fn create_string(value: &str) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Str(Arc::new(value.to_string())))
    }

    fn create_date(value: &str) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Date(Arc::new(
            NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap(),
        )))
    }

    fn create_datetime(value: &str) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::DateTime(Arc::new(
            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").unwrap(),
        )))
    }

    fn create_time(value: &str) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Time(Arc::new(
            NaiveTime::parse_from_str(value, "%H:%M:%S").unwrap(),
        )))
    }

    #[allow(dead_code)]
    fn create_bool(value: bool) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Bool(value))
    }

    fn create_vec(values: Vec<Arc<ArcTopicDataValue>>) -> Arc<ArcTopicDataValue> {
        Arc::new(ArcTopicDataValue::Vec(Arc::new(values)))
    }

    #[test]
    fn test_compute_count_vec() {
        let path = create_test_path(VariablePredefineFunctions::Count);
        let context = create_vec(vec![
            create_string("a"),
            create_string("b"),
            create_string("c"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(3));
        }
    }

    #[test]
    fn test_compute_count_map() {
        let path = create_test_path(VariablePredefineFunctions::Count);
        let mut map = std::collections::HashMap::new();
        map.insert("key1".to_string(), create_string("value1"));
        map.insert("key2".to_string(), create_string("value2"));
        let context = Arc::new(ArcTopicDataValue::Map(Arc::new(map)));
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(2));
        }
    }

    #[test]
    fn test_compute_length_string() {
        let path = create_test_path(VariablePredefineFunctions::Length);
        let context = create_string("hello");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(5));
        }
    }

    #[test]
    fn test_compute_length_number() {
        let path = create_test_path(VariablePredefineFunctions::Length);
        let context = create_decimal("12345");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(5));
        }
    }

    #[test]
    fn test_compute_slice() {
        let path = create_test_path(VariablePredefineFunctions::Slice);
        let context = create_string("hello world");
        let params = vec![create_decimal("0"), create_decimal("5")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello");
        }
    }

    #[test]
    fn test_compute_substr() {
        let path = create_test_path(VariablePredefineFunctions::Substr);
        let context = create_string("hello world");
        let params = vec![create_decimal("6"), create_decimal("5")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "world");
        }
    }

    #[test]
    fn test_compute_find() {
        let path = create_test_path(VariablePredefineFunctions::Find);
        let context = create_string("hello world");
        let params = vec![create_string("world")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(6));
        }
    }

    #[test]
    fn test_compute_index() {
        let path = create_test_path(VariablePredefineFunctions::Index);
        let context = create_string("hello world");
        let params = vec![create_string("hello")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(0));
        }
    }

    #[test]
    fn test_compute_starts_with() {
        let path = create_test_path(VariablePredefineFunctions::StartsWith);
        let context = create_string("hello world");
        let params = vec![create_string("hello")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Bool(_)));

        if let ArcTopicDataValue::Bool(b) = result.deref() {
            assert_eq!(*b, true);
        }
    }

    #[test]
    fn test_compute_ends_with() {
        let path = create_test_path(VariablePredefineFunctions::EndsWith);
        let context = create_string("hello world");
        let params = vec![create_string("world")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Bool(_)));

        if let ArcTopicDataValue::Bool(b) = result.deref() {
            assert_eq!(*b, true);
        }
    }

    #[test]
    fn test_compute_trim() {
        let path = create_test_path(VariablePredefineFunctions::Trim);
        let context = create_string("  hello  ");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello");
        }
    }

    #[test]
    fn test_compute_upper() {
        let path = create_test_path(VariablePredefineFunctions::Upper);
        let context = create_string("hello");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "HELLO");
        }
    }

    #[test]
    fn test_compute_lower() {
        let path = create_test_path(VariablePredefineFunctions::Lower);
        let context = create_string("HELLO");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello");
        }
    }

    #[test]
    fn test_compute_contains() {
        let path = create_test_path(VariablePredefineFunctions::Contains);
        let context = create_string("hello world");
        let params = vec![create_string("world")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Bool(_)));

        if let ArcTopicDataValue::Bool(b) = result.deref() {
            assert_eq!(*b, true);
        }
    }

    #[test]
    fn test_compute_split() {
        let path = create_test_path(VariablePredefineFunctions::Split);
        let context = create_string("a,b,c");
        let params = vec![create_string(",")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Vec(_)));

        if let ArcTopicDataValue::Vec(vec) = result.deref() {
            assert_eq!(vec.len(), 3);
        }
    }

    #[test]
    fn test_compute_concat() {
        let path = create_test_path(VariablePredefineFunctions::Concat);
        let context = create_string("hello");
        let params = vec![create_string(" "), create_string("world")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello world");
        }
    }

    #[test]
    fn test_compute_concat_with() {
        let path = create_test_path(VariablePredefineFunctions::ConcatWith);
        let context = create_string("hello");
        let params = vec![create_string("-"), create_string("world")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello-world");
        }
    }

    #[test]
    fn test_compute_join() {
        let path = create_test_path(VariablePredefineFunctions::Join);
        let context = create_vec(vec![
            create_string("a"),
            create_string("b"),
            create_string("c"),
        ]);
        let params = vec![create_string(",")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "a,b,c");
        }
    }

    #[test]
    fn test_compute_distinct() {
        let path = create_test_path(VariablePredefineFunctions::Distinct);
        let context = create_vec(vec![
            create_string("a"),
            create_string("b"),
            create_string("a"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Vec(_)));

        if let ArcTopicDataValue::Vec(vec) = result.deref() {
            assert_eq!(vec.len(), 2);
        }
    }

    #[test]
    fn test_compute_sum() {
        let path = create_test_path(VariablePredefineFunctions::Sum);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("2"),
            create_decimal("3"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(6));
        }
    }

    #[test]
    fn test_compute_avg() {
        let path = create_test_path(VariablePredefineFunctions::Avg);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("2"),
            create_decimal("3"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from_str("2").unwrap());
        }
    }

    #[test]
    fn test_compute_max() {
        let path = create_test_path(VariablePredefineFunctions::Max);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("3"),
            create_decimal("2"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(3));
        }
    }

    #[test]
    fn test_compute_max_num() {
        let path = create_test_path(VariablePredefineFunctions::MaxNum);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("3"),
            create_decimal("2"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(3));
        }
    }

    #[test]
    fn test_compute_max_date() {
        let path = create_test_path(VariablePredefineFunctions::MaxDate);
        let context = create_vec(vec![
            create_date("2024-01-01"),
            create_date("2024-01-02"),
            create_date("2024-01-03"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Date(_)));

        if let ArcTopicDataValue::Date(date) = result.deref() {
            assert_eq!(
                date.deref(),
                &NaiveDate::parse_from_str("2024-01-03", "%Y-%m-%d").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_max_datetime() {
        let path = create_test_path(VariablePredefineFunctions::MaxDt);
        let context = create_vec(vec![
            create_datetime("2024-01-01 10:00:00"),
            create_datetime("2024-01-01 11:00:00"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::DateTime(_)));

        if let ArcTopicDataValue::DateTime(dt) = result.deref() {
            assert_eq!(
                dt.deref(),
                &NaiveDateTime::parse_from_str("2024-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_max_time() {
        let path = create_test_path(VariablePredefineFunctions::MaxTime);
        let context = create_vec(vec![
            create_time("10:00:00"),
            create_time("12:00:00"),
            create_time("11:00:00"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Time(_)));

        if let ArcTopicDataValue::Time(time) = result.deref() {
            assert_eq!(
                time.deref(),
                &NaiveTime::parse_from_str("12:00:00", "%H:%M:%S").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_min() {
        let path = create_test_path(VariablePredefineFunctions::Min);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("3"),
            create_decimal("2"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(1));
        }
    }

    #[test]
    fn test_compute_min_num() {
        let path = create_test_path(VariablePredefineFunctions::MinNum);
        let context = create_vec(vec![
            create_decimal("1"),
            create_decimal("3"),
            create_decimal("2"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(1));
        }
    }

    #[test]
    fn test_compute_min_date() {
        let path = create_test_path(VariablePredefineFunctions::MinDate);
        let context = create_vec(vec![
            create_date("2024-01-01"),
            create_date("2024-01-02"),
            create_date("2024-01-03"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Date(_)));

        if let ArcTopicDataValue::Date(date) = result.deref() {
            assert_eq!(
                date.deref(),
                &NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_min_datetime() {
        let path = create_test_path(VariablePredefineFunctions::MinDt);
        let context = create_vec(vec![
            create_datetime("2024-01-01 10:00:00"),
            create_datetime("2024-01-01 11:00:00"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::DateTime(_)));

        if let ArcTopicDataValue::DateTime(dt) = result.deref() {
            assert_eq!(
                dt.deref(),
                &NaiveDateTime::parse_from_str("2024-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_min_time() {
        let path = create_test_path(VariablePredefineFunctions::MinTime);
        let context = create_vec(vec![
            create_time("10:00:00"),
            create_time("12:00:00"),
            create_time("11:00:00"),
        ]);
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Time(_)));

        if let ArcTopicDataValue::Time(time) = result.deref() {
            assert_eq!(
                time.deref(),
                &NaiveTime::parse_from_str("10:00:00", "%H:%M:%S").unwrap()
            );
        }
    }

    #[test]
    fn test_compute_replace() {
        let path = create_test_path(VariablePredefineFunctions::Replace);
        let context = create_string("hello world world");
        let params = vec![create_string("world"), create_string("universe")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello universe universe");
        }
    }

    #[test]
    fn test_compute_replace_first() {
        let path = create_test_path(VariablePredefineFunctions::ReplaceFirst);
        let context = create_string("hello world world");
        let params = vec![create_string("world"), create_string("universe")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "hello universe world");
        }
    }

    #[test]
    fn test_compute_not_enough_params() {
        let path = create_test_path(VariablePredefineFunctions::Slice);
        let context = create_string("hello");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_too_many_params() {
        let path = create_test_path(VariablePredefineFunctions::Slice);
        let context = create_string("hello");
        let params = vec![
            create_decimal("0"),
            create_decimal("5"),
            create_decimal("10"),
        ];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_context_disallowed() {
        let path = create_test_path(VariablePredefineFunctions::NextSeq);
        let context = create_string("test");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_day_diff() {
        let path = create_test_path(VariablePredefineFunctions::DayDiff);
        let context = create_datetime("2024-01-02 00:00:00");
        let params = vec![create_datetime("2024-01-01 00:00:00")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(1));
        }
    }

    #[test]
    fn test_compute_month_diff() {
        let path = create_test_path(VariablePredefineFunctions::MonthDiff);
        let context = create_datetime("2024-02-01 00:00:00");
        let params = vec![create_datetime("2024-01-01 00:00:00")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(1));
        }
    }

    #[test]
    fn test_compute_year_diff() {
        let path = create_test_path(VariablePredefineFunctions::YearDiff);
        let context = create_datetime("2025-01-01 00:00:00");
        let params = vec![create_datetime("2024-01-01 00:00:00")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Num(_)));

        if let ArcTopicDataValue::Num(num) = result.deref() {
            assert_eq!(num.deref(), &BigDecimal::from(1));
        }
    }

    #[test]
    fn test_compute_move_date() {
        let path = create_test_path(VariablePredefineFunctions::MoveDate);
        let context = create_datetime("2024-01-01 00:00:00");
        let params = vec![
            create_string("Y+1"),
            create_string("M+1"),
            create_string("D+1"),
        ];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::DateTime(_)));
    }

    #[test]
    fn test_compute_date_format() {
        let path = create_test_path(VariablePredefineFunctions::DateFormat);
        let context = create_datetime("2024-01-01 12:30:45");
        let params = vec![create_string("yyyy-MM-dd HH:mm:ss")];

        let result = InMemoryFuncCall::compute(&path, context, params).unwrap();
        assert!(matches!(result.deref(), ArcTopicDataValue::Str(_)));

        if let ArcTopicDataValue::Str(s) = result.deref() {
            assert_eq!(s.as_str(), "2024-01-01 12:30:45");
        }
    }

    #[test]
    fn test_compute_now_context_disallowed() {
        let path = create_test_path(VariablePredefineFunctions::Now);
        let context = create_string("test");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_from_current_trigger_data_disallowed() {
        let path = create_test_path(VariablePredefineFunctions::FromCurrentTriggerData);
        let context = create_string("test");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_from_previous_trigger_data_disallowed() {
        let path = create_test_path(VariablePredefineFunctions::FromPreviousTriggerData);
        let context = create_string("test");
        let params = vec![];

        let result = InMemoryFuncCall::compute(&path, context, params);
        assert!(result.is_err());
    }
}
