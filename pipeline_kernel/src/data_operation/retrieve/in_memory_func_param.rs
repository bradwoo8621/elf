use crate::{ArcFrom, ArcTopicDataValue, FuncDataPathParam, FuncParamValue, InMemoryData};
use elf_base::StdR;
use std::sync::Arc;

impl FuncDataPathParam {
    pub fn value_from_memory(&self, in_memory_data: &InMemoryData) -> StdR<Arc<ArcTopicDataValue>> {
        match self {
            Self::Path(data_path) => in_memory_data.value_of(data_path),
            Self::Plain(plain_path) => plain_path.value_from_memory(in_memory_data),
            Self::Func(func_path) => func_path.value_from_memory(in_memory_data),
            Self::Value(data_path_param) => {
                let value = data_path_param.value();
                match value {
                    FuncParamValue::Str(s) => Ok(ArcTopicDataValue::arc_from(s.clone())),
                    FuncParamValue::Num(n) => Ok(ArcTopicDataValue::arc_from(n.clone())),
                    FuncParamValue::Bool(b) => Ok(ArcTopicDataValue::arc_from(*b)),
                    FuncParamValue::DateTime(dt) => Ok(ArcTopicDataValue::arc_from(dt.clone())),
                    FuncParamValue::Date(d) => Ok(ArcTopicDataValue::arc_from(d.clone())),
                    FuncParamValue::Time(t) => Ok(ArcTopicDataValue::arc_from(t.clone())),
                    FuncParamValue::None => Ok(Arc::new(ArcTopicDataValue::None)),
                }
            }
        }
    }
}
