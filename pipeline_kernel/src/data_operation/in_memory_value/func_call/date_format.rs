use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall, FUNC_PARAM_TRANSFORMED_TAG};
use elf_base::{DateTimeFormatTransformSupport, DateTimeUtils, StdR};
use std::ops::Deref;
use std::sync::Arc;

struct DateFormatHelper;

impl DateFormatHelper {
    fn format_date(
        func_call: &InMemoryFuncCall,
        value: &ArcTopicDataValue,
        format: &ArcTopicDataValue,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let formatted = match value {
            ArcTopicDataValue::Date(date) => date
                .format(Self::get_format_from_param(func_call, format)?.as_str())
                .to_string(),
            ArcTopicDataValue::DateTime(datetime) => datetime
                .format(Self::get_format_from_param(func_call, format)?.as_str())
                .to_string(),
            ArcTopicDataValue::Time(time) => time
                .format(Self::get_format_from_param(func_call, format)?.as_str())
                .to_string(),
            ArcTopicDataValue::Str(str) => {
                if let Ok(datetime) = str.to_datetime_loose() {
                    datetime
                        .format(Self::get_format_from_param(func_call, format)?.as_str())
                        .to_string()
                } else if let Ok(time) = str.to_time() {
                    time.format(Self::get_format_from_param(func_call, format)?.as_str())
                        .to_string()
                } else {
                    return func_call.func_not_supported(str);
                }
            }
            other => return func_call.func_not_supported(other),
        };
        Ok(ArcTopicDataValue::arc_from(formatted))
    }

    fn get_format_from_param(
        func_call: &InMemoryFuncCall,
        format: &ArcTopicDataValue,
    ) -> StdR<Arc<String>> {
        match format {
            ArcTopicDataValue::Str(str) => {
                Ok(Arc::new(DateTimeFormatTransformSupport::transform(str)))
            }
            ArcTopicDataValue::Vec(vec) => {
                if vec.len() != 2 {
                    return func_call.func_not_supported(format);
                }
                match (vec[0].deref(), vec[1].deref()) {
                    (ArcTopicDataValue::Str(tag), ArcTopicDataValue::Str(s)) => {
                        if tag.deref() != FUNC_PARAM_TRANSFORMED_TAG {
                            func_call.func_not_supported(format)
                        } else {
                            Ok(s.clone())
                        }
                    }
                    _ => func_call.func_not_supported(format),
                }
            }
            other => func_call.func_not_supported(other),
        }
    }
}

impl InMemoryFuncCall<'_> {
    pub fn resolve_date_format(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.one_param(&params, |param| {
            DateFormatHelper::format_date(&self, context.deref(), param)
        })
    }
}
