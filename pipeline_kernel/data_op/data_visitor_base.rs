use crate::{
    ArcTopicData, ArcTopicDataValue, DataPath, DataPathSegment, PipelineKernelErrorCode,
    PlainDataPath, VariablePredefineFunctionCaller,
};
use elf_base::{ErrorCode, StdR};
use std::ops::Deref;
use std::sync::Arc;

pub trait DataVisitorBase {
    fn value_of_path(&self, parsed_path: &DataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitorBase for ArcTopicData {
    fn value_of_path(&self, parsed_path: &DataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let path = &parsed_path.path.to_string();
        let mut data = Arc::new(ArcTopicDataValue::Map(self.clone()));
        for segment in &parsed_path.segments {
            let current_is_vec = match segment {
                DataPathSegment::Func(segment) => {
                    data = VariablePredefineFunctionCaller::prepare(&self, path, segment)
                        .value_of(&data)?;
                    // never mind, just keep the value which returned, no need to do post transforming
                    false
                }
                DataPathSegment::Plain(segment) => {
                    data = self.value_of_plain_segment(&data, segment, path)?;
                    segment.is_vec.unwrap_or(false)
                }
            };

            // recheck the data, is there none, empty vec, then there is no need to go deeper.
            // and when current segment says the value should be a vec, convert none to empty vec
            // and return directly
            match data.deref() {
                ArcTopicDataValue::None => {
                    return if current_is_vec {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    };
                }
                ArcTopicDataValue::Vec(vec) => {
                    if vec.is_empty() {
                        return Ok(data.clone());
                    }
                }
                _ => {}
            }
        }

        Ok(data.clone())
    }
}
