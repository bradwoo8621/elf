use crate::{DataPath, DataPathSegment, ParsedDataPath, PipelineKernelErrorCode, PlainDataPath};
use std::ops::Deref;
use watchmen_model::{FactorType, StdErrorCode, StdR};
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

        if segments.len() == 1 {
            Ok(DataPath::Simple(ParsedDataPath {
                path: factor.name.deref().clone(),
                segments,
            }))
        } else {
            Ok(DataPath::Complex(ParsedDataPath {
                path: factor.name.deref().clone(),
                segments,
            }))
        }
    }
}
