use crate::{
    DataPath, DataPathSegment, PathParser, PathStr, PipelineKernelErrorCode, PlainDataPath,
};
use std::sync::Arc;
use watchmen_model::{FactorType, StdErrorCode, StdR};
use watchmen_runtime_model_kernel::{ArcFactor, TopicSchema};

/// parser
impl DataPath {
    /// factor name has no dot escape, no function
    pub fn from_factor(factor: &ArcFactor, topic_schema: &TopicSchema) -> StdR<DataPath> {
        let mut segments = vec![];
        let segment_paths: Vec<&str> = factor.name.split('.').collect();
        for (index, _) in segment_paths.iter().enumerate() {
            // each path is from start
            let path = segment_paths[0..(index + 1)].join(".");
            let factor = topic_schema.factor_by_name(&path);
            let is_vec = if let Some(factor) = factor {
                *factor.r#type.as_ref() == FactorType::Array
            } else {
                return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                    "Factor[{}] not found in topic[{}].",
                    &path,
                    topic_schema.topic_id()
                ));
            };
            segments.push(DataPathSegment::Plain(PlainDataPath {
                path: PathStr::of_str(segment_paths[index]),
                is_vec: Some(is_vec),
            }));
        }

        Ok(DataPath {
            path: PathStr::of_str(factor.name.as_str()),
            segments,
        })
    }

    /// all kinds escape chars, functions, variables
    /// - \. escape of dot,
    /// - \, escape of comma,
    /// - \( escape of left parenthesis,
    /// - \) escape of right parenthesis,
    /// - \{ escape of left brace,
    /// - \} escape of right brace,
    /// - \& escape of ampersand,
    /// - \t escape of tab,
    /// - \r escape of carriage return,
    /// - \n escape of newline,
    /// - abc{ef} escapes path [ef],
    ///
    /// and fail fast
    pub fn from_str(path: &str) -> StdR<DataPath> {
        let all_chars: Arc<Vec<char>> = Arc::new(path.chars().collect());
        let mut parser = PathParser::by_path(all_chars.clone());
        parser.parse()?;

        Ok(DataPath {
            path: PathStr::of_chars(all_chars),
            segments: parser.segments,
        })
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    mod helper {
        use crate::{DataPathSegment, FuncDataPathParam, FuncParamValue};
        use watchmen_model::VariablePredefineFunctions;

        pub fn assert_plain_segment(segment: &DataPathSegment, value: &str) {
            assert!(matches!(segment, DataPathSegment::Plain(_)));
            match segment {
                DataPathSegment::Plain(path) => {
                    assert_eq!(path.path.to_string(), value);
                    assert_eq!(path.is_vec, None);
                }
                _ => panic!(),
            }
        }

        pub fn assert_param_plain(param: &FuncDataPathParam, value: &str) {
            assert!(matches!(param, FuncDataPathParam::Plain(_)));
            match param {
                FuncDataPathParam::Plain(plain_path) => {
                    assert_eq!(plain_path.path.to_string(), value);
                    assert_eq!(plain_path.is_vec, None);
                }
                _ => panic!(),
            }
        }

        pub fn assert_param_str(param: &FuncDataPathParam, value: &str) {
            match param {
                FuncDataPathParam::Value(value_path) => {
                    assert_eq!(value_path.path.to_string(), value);
                    assert!(matches!(value_path.value, FuncParamValue::Str(_)));
                    match &value_path.value {
                        FuncParamValue::Str(s) => {
                            assert_eq!(s, value);
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }

        pub fn assert_param_none(param: &FuncDataPathParam, value: &str) {
            match param {
                FuncDataPathParam::Value(value_path) => {
                    assert_eq!(value_path.path.to_string(), value);
                    assert!(matches!(value_path.value, FuncParamValue::None));
                }
                _ => panic!(),
            }
        }

        pub fn assert_func_segment<F1, F2>(segment: &DataPathSegment, path: &str, f1: F1, f2: F2)
        where
            F1: FnOnce(&VariablePredefineFunctions),
            F2: FnOnce(&Vec<FuncDataPathParam>),
        {
            assert!(matches!(segment, DataPathSegment::Func(_)));
            match segment {
                DataPathSegment::Func(func_path) => {
                    assert_eq!(func_path.path.to_string(), path);
                    f1(&func_path.func);
                    assert!(func_path.params.is_some());
                    if let Some(params) = &func_path.params {
                        f2(params);
                    }
                }
                _ => {}
            }
        }

        pub fn assert_func_no_param_segment<F1>(segment: &DataPathSegment, path: &str, f1: F1)
        where
            F1: FnOnce(&VariablePredefineFunctions),
        {
            assert!(matches!(segment, DataPathSegment::Func(_)));
            match segment {
                DataPathSegment::Func(func_path) => {
                    assert_eq!(func_path.path.to_string(), path);
                    f1(&func_path.func);
                    assert!(func_path.params.is_none());
                }
                _ => {}
            }
        }
    }

    mod plain {
        use crate::data_op::data_path_parser::tests::helper::assert_plain_segment;
        use crate::DataPath;

        #[test]
        fn test__a() {
            println!("test__a");

            let path = DataPath::from_str("a").unwrap();
            assert_eq!(path.path.to_string(), "a");
            assert_eq!(path.segments.len(), 1);
            assert_plain_segment(&path.segments[0], "a");

            println!("[a] parse to {}", path)
        }

        #[test]
        fn test__a_b() {
            println!("test__a_b");

            let path = DataPath::from_str("a.b").unwrap();
            assert_eq!(path.path.to_string(), "a.b");
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_plain_segment(&path.segments[1], "b");

            println!("[a.b] parse to {}", path)
        }

        #[test]
        fn test__a_b_c() {
            println!("test__a_b_c");

            let path = DataPath::from_str("a.b.c").unwrap();
            assert_eq!(path.path.to_string(), "a.b.c");
            assert_eq!(path.segments.len(), 3);
            assert_plain_segment(&path.segments[0], "a");
            assert_plain_segment(&path.segments[1], "b");
            assert_plain_segment(&path.segments[2], "c");

            println!("[a.b.c] parse to {}", path)
        }
    }

    mod literal_concat {
        use crate::data_op::data_path_parser::tests::helper::{
            assert_func_segment, assert_param_plain, assert_param_str,
        };
        use crate::DataPath;
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__LBaRB() {
            println!("test__LBaRB");

            let path = DataPath::from_str("{a}").unwrap();
            assert_eq!(path.path.to_string(), "{a}");
            assert_eq!(path.segments.len(), 1);
            assert_func_segment(
                &path.segments[0],
                "{a}",
                |f| assert!(matches!(f, VariablePredefineFunctions::Concat)),
                |params| {
                    assert_eq!(params.len(), 1);
                    assert_param_plain(&params[0], "a");
                },
            );

            println!("[{{a}}] parse to {}", path)
        }

        #[test]
        fn test__LBaRBb() {
            println!("test__LBaRBb");

            let path = DataPath::from_str("{a}b").unwrap();
            assert_eq!(path.path.to_string(), "{a}b");
            assert_eq!(path.segments.len(), 1);
            assert_func_segment(
                &path.segments[0],
                "{a}b",
                |f| assert!(matches!(f, VariablePredefineFunctions::Concat)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_plain(&params[0], "a");
                    assert_param_str(&params[1], "b");
                },
            );

            println!("[{{a}}b] parse to {}", path)
        }

        #[test]
        fn test__aLBbRB() {
            println!("test__aLBbRB");

            let path = DataPath::from_str("a{b}").unwrap();
            assert_eq!(path.path.to_string(), "a{b}");
            assert_eq!(path.segments.len(), 1);
            assert_func_segment(
                &path.segments[0],
                "a{b}",
                |f| assert!(matches!(f, VariablePredefineFunctions::Concat)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_str(&params[0], "a");
                    assert_param_plain(&params[1], "b");
                },
            );

            println!("[a{{b}}] parse to {}", path)
        }

        #[test]
        fn test__aLBbRBc() {
            println!("test__aLBbRBc");

            let path = DataPath::from_str("a{b}c").unwrap();
            assert_eq!(path.path.to_string(), "a{b}c");
            assert_eq!(path.segments.len(), 1);
            assert_func_segment(
                &path.segments[0],
                "a{b}c",
                |f| assert!(matches!(f, VariablePredefineFunctions::Concat)),
                |params| {
                    assert_eq!(params.len(), 3);
                    assert_param_str(&params[0], "a");
                    assert_param_plain(&params[1], "b");
                    assert_param_str(&params[2], "c");
                },
            );

            println!("[a{{b}}c] parse to {}", path)
        }
    }

    mod plain__literal_concat {
        use crate::data_op::data_path_parser::tests::helper::{
            assert_func_segment, assert_param_plain, assert_param_str, assert_plain_segment,
        };
        use crate::DataPath;
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__a_b_c_dLBeRBf_g() {
            println!("test__a_b_c_dLBeRBf_g");

            let path = DataPath::from_str("a.b.c.d{e}f.g").unwrap();
            assert_eq!(path.path.to_string(), "a.b.c.d{e}f.g");
            assert_eq!(path.segments.len(), 5);
            assert_plain_segment(&path.segments[0], "a");
            assert_plain_segment(&path.segments[1], "b");
            assert_plain_segment(&path.segments[2], "c");
            assert_func_segment(
                &path.segments[3],
                "d{e}f",
                |f| assert!(matches!(f, VariablePredefineFunctions::Concat)),
                |params| {
                    assert_eq!(params.len(), 3);
                    assert_param_str(&params[0], "d");
                    assert_param_plain(&params[1], "e");
                    assert_param_str(&params[2], "f");
                },
            );
            assert_plain_segment(&path.segments[4], "g");

            println!("[a.b.c.d{{e}}f.g] parse to {}", path);
        }
    }

    mod func_now {
        use crate::data_op::data_path_parser::tests::helper::assert_func_no_param_segment;
        use crate::DataPath;
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__now() {
            println!("test__now");

            let path = DataPath::from_str("&now").unwrap();
            assert_eq!(path.path.to_string(), "&now");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 4);
            assert_eq!(path.segments.len(), 1);
            assert_func_no_param_segment(&path.segments[0], "&now", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Now))
            });

            println!("[&now] parse to {}", path)
        }

        #[test]
        fn test__nowLPRP() {
            println!("test__nowLPRP");

            let path = DataPath::from_str("&now()").unwrap();
            assert_eq!(path.path.to_string(), "&now()");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 6);
            assert_eq!(path.segments.len(), 1);
            assert_func_no_param_segment(&path.segments[0], "&now()", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Now))
            });

            println!("[&now()] parse to {}", path)
        }

        #[test]
        fn test__nowLPWsRP() {
            println!("test__nowLPWsRP");

            let path = DataPath::from_str("&now(   )").unwrap();
            assert_eq!(path.path.to_string(), "&now(   )");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 9);
            assert_eq!(path.segments.len(), 1);
            assert_func_no_param_segment(&path.segments[0], "&now(   )", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Now))
            });

            println!("[&now(   )] parse to {}", path)
        }
    }

    mod func_len {
        use crate::data_op::data_path_parser::tests::helper::{
            assert_func_no_param_segment, assert_func_segment, assert_param_plain,
            assert_plain_segment,
        };
        use crate::DataPath;
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__a_len() {
            println!("test__a_len");

            let path = DataPath::from_str("a.&len").unwrap();
            assert_eq!(path.path.to_string(), "a.&len");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 6);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_no_param_segment(&path.segments[1], "&len", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Len))
            });

            println!("[a.&len] parse to {}", path)
        }

        #[test]
        fn test__a_lenLPRP() {
            println!("test__a_lenLPRP");

            let path = DataPath::from_str("a.&len()").unwrap();
            assert_eq!(path.path.to_string(), "a.&len()");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 8);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_no_param_segment(&path.segments[1], "&len()", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Len))
            });

            println!("[a.&len()] parse to {}", path)
        }

        #[test]
        fn test__a_lenLPWsRP() {
            println!("test__a_lenLPWsRP");

            let path = DataPath::from_str("a.&len(   )").unwrap();
            assert_eq!(path.path.to_string(), "a.&len(   )");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 11);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_no_param_segment(&path.segments[1], "&len(   )", |f| {
                assert!(matches!(f, VariablePredefineFunctions::Len))
            });

            println!("[a.&len(   )] parse to {}", path)
        }

        #[test]
        fn test__lenLPaRP() {
            println!("test__lenLPaRP");

            let path = DataPath::from_str("&len(a)").unwrap();
            assert_eq!(path.path.to_string(), "&len(a)");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 7);
            assert_eq!(path.segments.len(), 1);
            assert_func_segment(
                &path.segments[0],
                "&len(a)",
                |f| assert!(matches!(f, VariablePredefineFunctions::Len)),
                |params| {
                    assert_eq!(params.len(), 1);
                    assert_param_plain(&params[0], "a");
                },
            );

            println!("[&len(a)] parse to {}", path);
        }
    }

    mod func_slice {
        use crate::data_op::data_path_parser::tests::helper::{assert_func_segment, assert_param_none, assert_param_plain, assert_plain_segment};
        use crate::DataPath;
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__a_sliceLP1C2RP() {
            println!("test__a_sliceLP1C2RP");

            let path = DataPath::from_str("a.&slice(1,2)").unwrap();
            assert_eq!(path.path.to_string(), "a.&slice(1,2)");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 13);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_segment(
                &path.segments[1],
                "&slice(1,2)",
                |f| assert!(matches!(f, VariablePredefineFunctions::Slice)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_plain(&params[0], "1");
                    assert_param_plain(&params[1], "2");
                },
            );

            println!("[a.&slice(1,2)] parse to {}", path)
        }

        #[test]
        fn test__a_sliceLPNonC2RP() {
            println!("test__a_sliceLPNonC2RP");

            let path = DataPath::from_str("a.&slice(,2)").unwrap();
            assert_eq!(path.path.to_string(), "a.&slice(,2)");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 12);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_segment(
                &path.segments[1],
                "&slice(,2)",
                |f| assert!(matches!(f, VariablePredefineFunctions::Slice)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_none(&params[0], "");
                    assert_param_plain(&params[1], "2");
                },
            );

            println!("[a.&slice(,2)] parse to {}", path)
        }

        #[test]
        fn test__a_sliceLP1CNonRP() {
            println!("test__a_sliceLP1CNonRP");

            let path = DataPath::from_str("a.&slice(1,)").unwrap();
            assert_eq!(path.path.to_string(), "a.&slice(1,)");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 12);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_segment(
                &path.segments[1],
                "&slice(1,)",
                |f| assert!(matches!(f, VariablePredefineFunctions::Slice)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_plain(&params[0], "1");
                    assert_param_none(&params[1], "");
                },
            );

            println!("[a.&slice(1,)] parse to {}", path)
        }

        #[test]
        fn test__a_sliceLPNonCNonRP() {
            println!("test__a_sliceLPNonCNonRP");

            let path = DataPath::from_str("a.&slice(,)").unwrap();
            assert_eq!(path.path.to_string(), "a.&slice(,)");
            assert_eq!(path.path.start_index(), 0);
            assert_eq!(path.path.end_index(), 11);
            assert_eq!(path.segments.len(), 2);
            assert_plain_segment(&path.segments[0], "a");
            assert_func_segment(
                &path.segments[1],
                "&slice(,)",
                |f| assert!(matches!(f, VariablePredefineFunctions::Slice)),
                |params| {
                    assert_eq!(params.len(), 2);
                    assert_param_none(&params[0], "");
                    assert_param_none(&params[1], "");
                },
            );

            println!("[a.&slice(,)] parse to {}", path)
        }
    }
}
