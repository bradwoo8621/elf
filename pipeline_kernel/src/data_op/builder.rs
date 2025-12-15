use crate::{DataPath, DataPathSegment, PipelineKernelErrorCode, PlainDataPath};
use std::ops::Deref;
use watchmen_model::{FactorType, StdErrorCode, StdR, StringUtils};
use watchmen_runtime_model_kernel::{ArcFactor, TopicSchema};

pub struct DataPathBuilder;

impl DataPathBuilder {
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

    /// check the given char can be escaped or not
    /// if yes, append the escaped char to given str, return true
    /// otherwise append char '\' to given str, return false
    fn try_consume_escape_char(chars: &Vec<char>, current_index: usize, str: &mut String) -> bool {
        if let Some(next_c) = chars.get(current_index + 1) {
            match next_c {
                '.' => str.push('.'),
                ',' => str.push(','),
                '(' => str.push('('),
                ')' => str.push(')'),
                '{' => str.push('{'),
                '}' => str.push('}'),
                '&' => str.push('&'),
                _ => {
                    str.push('\\');
                    return false;
                }
            }
            true
        } else {
            str.push('\\');
            false
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
        let chars: Vec<char> = path.chars().collect();

        let mut current_str = String::new();
        let mut ignore_next_chars = 0;
        let mut brace_depth = 0;

        for index in 0..chars.len() - 1 {
            if ignore_next_chars != 0 {
                ignore_next_chars -= 1;
                continue;
            }

            let c = chars[index];
            match c {
                '\\' => {
                    // check escapes, ignore next char when it is an escape (next char consumed)
                    if Self::try_consume_escape_char(&chars, index, &mut current_str) {
                        ignore_next_chars = 1
                    }
                }
                '.' => {
                    // segment end, new segment will start by next char
                    if current_str.is_blank() {
                        // nothing captured of this segment, error
                        return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                            "Cannot parse path[{}], caused by blank segment determined at index[{}].",
                            path, index
                        ));
                    } else {
                        // TODO
                    }
                }
                '&' => {
                    // function starts, must after one of [.,({], or is start of whole path
                    if current_str.is_blank() {
                        // blank string before function name, ignored
                        current_str.clear();
                        current_str.push('&');
                    } else {
                        // no content allowed before function name in this segment
                        return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                            "Cannot parse path[{}], caused by content determined before function at index[{}].",
                            path, index
                        ));
                    }
                    // try to parse function name,
                    // the next char of function name should be whitespace or one of .,({ or is end of whole path
                    // TODO
                }
                _ => current_str.push(c),
            }
        }

        // TODO never mind, to avoid the compile error
        let path = path.to_string();
        Ok(DataPath {
            path: path.clone(),
            segments: vec![DataPathSegment::Plain(PlainDataPath { path, is_vec: None })],
        })
    }
}
