use crate::{
    DataPath, DataPathSegment, FuncDataPath, FuncDataPathParam, FuncDataPathParamPart,
    PipelineKernelErrorCode, PlainDataPath,
};
use std::ops::Deref;
use watchmen_model::{FactorType, StdErrorCode, StdR, StringUtils, VariablePredefineFunctions};
use watchmen_runtime_model_kernel::{ArcFactor, TopicSchema};

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
                path: segment_paths[index].to_string(),
                is_vec: Some(is_vec),
            }));
        }

        Ok(DataPath {
            path: factor.name.deref().clone(),
            segments,
        })
    }

    fn incorrect_dot<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect dot at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_comma<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect comma at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_left_parenthesis<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect left parenthesis at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_right_parenthesis<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect right parenthesis at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_left_brace<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect left brace at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_right_brace<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect right brace at index[{}].",
            path, char_index
        ))
    }

    fn incorrect_ampersand<R>(path: &str, char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect ampersand at index[{}].",
            path, char_index
        ))
    }

    /// start is included, end is excluded
    fn incorrect_blank_segment<R>(
        path: &str,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect ampersand at index[{}, {}].",
            path, start_char_index, end_char_index
        ))
    }

    /// check the given char can be escaped or not
    /// if yes, append the escaped char to given str, return char index after the escaped char, which is [char_index + 2].
    /// otherwise append char '\' to given str, return char index after [\], which is same as given [char_index + 1].
    ///
    /// the returned bool means reach the end of chars or not.
    fn try_consume_escape_char(
        chars: &Vec<char>,
        char_index: usize,
        current_chars: &mut String,
    ) -> (usize, bool) {
        let next_char_index = char_index + 1;
        if let Some(next_c) = chars.get(char_index + 1) {
            match next_c {
                '.' | ',' | '(' | ')' | '{' | '}' | '&' => {
                    current_chars.push(*next_c);
                    (next_char_index + 1, false)
                }
                _ => {
                    current_chars.push('\\');
                    (next_char_index, false)
                }
            }
        } else {
            current_chars.push('\\');
            (next_char_index, true)
        }
    }

    fn consume_plain_path(segments: &mut Vec<DataPathSegment>, current_chars: &String) {
        segments.push(DataPathSegment::Plain(PlainDataPath {
            path: current_chars.clone(),
            is_vec: None,
        }));
    }

    /// params:
    /// - [full_path]: full qualified path,
    /// - [chars]: chars of full qualified path,
    /// - [segments]: consumed segments before it, in same level,
    /// - [char_index]: index of start([&]),
    /// - [current_chars]: chars in memory after consumed previous segments
    ///
    /// > any chars before function is not allowed.
    fn try_consume_function(
        full_path: &str,
        chars: &Vec<char>,
        char_index: usize,
        current_chars: &mut String,
    ) -> StdR<(FuncDataPath, bool)> {
        if !current_chars.is_empty() {
            return DataPath::incorrect_ampersand(full_path, char_index);
        }

        current_chars.push('&');
        // TODO
        Ok((1, false))
    }

    /// path can contain multiple segments
    /// each segment can be,
    /// - starts with [&]: function,
    /// - not starts with [&], not contains [{}], plain,
    /// - not starts with [&], and contains [{}], function concat.
    /// and
    /// - cannot start with [().,],
    /// - cannot end with [.,],
    /// - before first [.], cannot be blank,
    /// - after last [.], cannot be blank,
    /// - for the literal concat function, functions that are not wrapped in `{}` are not allowed to appear.
    ///   e.g. [a.&len {b.len}] is not allowed.
    fn try_consume_path(
        full_path: &str,
        chars: &Vec<char>,
        start_char_index: usize,
        segments: &mut Vec<DataPathSegment>,
    ) -> StdR<(usize, bool)> {
        let mut char_index = start_char_index;
        let mut current_chars = String::new();
        loop {
            if let Some(char) = chars.get(char_index) {
                let next_char_index: usize;
                let end: bool;

                match char {
                    '&' => {
                        // start of function, no content before function
                        let (consumed, is_end) = DataPath::try_consume_function(
                            full_path,
                            chars,
                            char_index,
                            &mut current_chars,
                        )?;
                        next_char_index = char_index + consumed.path.len();
                        end = is_end;
                    }
                    '(' => return DataPath::incorrect_left_parenthesis(full_path, char_index),
                    ')' => return DataPath::incorrect_right_parenthesis(full_path, char_index),
                    '{' => {
                        // start of sub path
                        // check the last segment, convert it to concat
                        let mut params: Vec<FuncDataPathParam> = vec![];
                        if let Some(last_segment) = segments.pop() {
                            match last_segment {
                                DataPathSegment::Plain(data_path) => {
                                    params.push(FuncDataPathParam {
                                        path: data_path.path.clone(),
                                        part: FuncDataPathParamPart::Str(data_path.path),
                                    })
                                }
                                DataPathSegment::Func(data_path) => {
                                    params.push(FuncDataPathParam {
                                        path: data_path.path.clone(),
                                        part: FuncDataPathParamPart::Variable(
                                            DataPathSegment::Func(data_path),
                                        ),
                                    })
                                }
                            };
                        }
                        (next_char_index, end) = DataPath::try_consume_sub_path(
                            full_path,
                            &params,
                            char_index,
                            &mut current_chars,
                        )?;
                        segments.push(DataPathSegment::Func(FuncDataPath {
                            path: chars[char_index..next_char_index].iter().collect(),
                            func: VariablePredefineFunctions::Concat,
                            params: Some(params),
                        }));
                    }
                    '}' => return DataPath::incorrect_right_brace(full_path, char_index),
                    '.' => {
                        // segment end
                        if current_chars.is_blank() {
                            return DataPath::incorrect_blank_segment(
                                full_path,
                                start_char_index,
                                char_index,
                            );
                        } else {
                            DataPath::consume_plain_path(segments, &current_chars);
                            (next_char_index, end) = (char_index + 1, false);
                        }
                    }
                    ',' => return DataPath::incorrect_comma(full_path, char_index),
                    '\\' => {
                        (next_char_index, end) = DataPath::try_consume_escape_char(
                            &chars,
                            char_index,
                            &mut current_chars,
                        )
                    }
                    _ => {
                        current_chars.push(*char);
                        (next_char_index, end) = (char_index + 1, false);
                    }
                };
                if end {
                    break;
                } else {
                    char_index = next_char_index;
                }
            } else {
                // reach the end of chars
                break;
            }
        }
    }

    /// all kinds escapes, functions, variables
    /// - \. escapes dot,
    /// - \, escapes comma,
    /// - \( escapes left parenthesis,
    /// - \) escapes right parenthesis,
    /// - \{ escapes left brace,
    /// - \} escapes right brace,
    /// - \& escapes ampersand,
    /// - abc{ef} escapes path [ef],
    ///
    /// and fail fast
    pub fn from_str(path: &str) -> StdR<DataPath> {
        let mut data_path = DataPath {
            path: path.to_string(),
            segments: vec![],
        };
        DataPath::try_consume_path(path, &path.chars().collect(), 0, &mut data_path.segments)?;
        Ok(data_path)
    }
}
