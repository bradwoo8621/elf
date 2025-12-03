use crate::PipelineKernelErrorCode;
use std::sync::Arc;
use watchmen_model::{
    PipelineTriggerType, StdErrorCode, StdR, TopicData, TopicDataColumnNames, TopicDataId,
    TopicDataValue,
};

pub struct TopicTrigger {
    pub previous: Option<Arc<TopicData>>,
    pub current: Option<Arc<TopicData>>,
    pub r#type: PipelineTriggerType,
    pub internal_data_id: Arc<TopicDataId>,
}

impl TopicTrigger {
    fn get_data_id(data: &TopicData) -> StdR<Arc<TopicDataId>> {
        let data_id = data.get(TopicDataColumnNames::Id.column_name());
        match data_id {
            Some(data_id) => match data_id {
                TopicDataValue::Str(data_id) => Ok(Arc::new(data_id.clone())),
                TopicDataValue::Num(num) => Ok(Arc::new(num.to_string())),
                _ => PipelineKernelErrorCode::TopicDataIdTypeNotSupported.msg(format!(
                    "Topic data id type not supported, of data[{:?}].",
                    data
                )),
            },
            _ => PipelineKernelErrorCode::TopicDataIdNotFound
                .msg(format!("Topic data id not found, of data[{:?}].", data)),
        }
    }

    pub fn insert_to_synonym(current: TopicData) -> StdR<Arc<TopicTrigger>> {
        Ok(Arc::new(TopicTrigger {
            current: Some(Arc::new(current)),
            previous: None,
            r#type: PipelineTriggerType::Insert,
            internal_data_id: Arc::new("-1".to_string()),
        }))
    }

    pub fn insert(current: TopicData) -> StdR<Arc<TopicTrigger>> {
        let data_id = TopicTrigger::get_data_id(&current)?;

        Ok(Arc::new(TopicTrigger {
            current: Some(Arc::new(current)),
            previous: None,
            r#type: PipelineTriggerType::Insert,
            internal_data_id: data_id,
        }))
    }

    pub fn merge(previous: TopicData, current: TopicData) -> StdR<Arc<TopicTrigger>> {
        let data_id = TopicTrigger::get_data_id(&current)?;

        Ok(Arc::new(TopicTrigger {
            current: Some(Arc::new(current)),
            previous: Some(Arc::new(previous)),
            r#type: PipelineTriggerType::Merge,
            internal_data_id: data_id,
        }))
    }

    pub fn delete(previous: TopicData) -> StdR<Arc<TopicTrigger>> {
        let data_id = TopicTrigger::get_data_id(&previous)?;

        Ok(Arc::new(TopicTrigger {
            current: None,
            previous: Some(Arc::new(previous)),
            r#type: PipelineTriggerType::Delete,
            internal_data_id: data_id,
        }))
    }
}
