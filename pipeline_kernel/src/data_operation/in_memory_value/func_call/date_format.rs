use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::{DateTimeFormatTransformSupport, DateTimeUtils, StdR};
use std::ops::Deref;
use std::sync::Arc;

impl InMemoryFuncCall<'_> {
    pub fn resolve_date_format(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.one_param(&params, |param| match (context.deref(), param) {
            (ArcTopicDataValue::Date(date), ArcTopicDataValue::Str(f)) => {
                let format = DateTimeFormatTransformSupport::transform(f.deref());
                Ok(ArcTopicDataValue::arc_from(
                    date.format(format.as_str()).to_string(),
                ))
            }
            (ArcTopicDataValue::DateTime(datetime), ArcTopicDataValue::Str(f)) => {
                let format = DateTimeFormatTransformSupport::transform(f.deref());
                Ok(ArcTopicDataValue::arc_from(
                    datetime.format(format.as_str()).to_string(),
                ))
            }
            (ArcTopicDataValue::Str(str), ArcTopicDataValue::Str(f)) => {
                if let Ok(datetime) = str.to_datetime_loose() {
                    let format = DateTimeFormatTransformSupport::transform(f.deref());
                    Ok(ArcTopicDataValue::arc_from(
                        datetime.format(format.as_str()).to_string(),
                    ))
                } else {
                    self.func_not_supported(context.deref())
                }
            }
            (_, ArcTopicDataValue::Str(_)) => self.func_not_supported(context.deref()),
            _ => self.func_not_supported(param),
        })
    }
}
