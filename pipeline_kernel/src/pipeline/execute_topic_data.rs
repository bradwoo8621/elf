use crate::{ArcTopicData, ArcTopicDataBuilder, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdR};
use elf_model::{TopicData, TopicDataColumnNames, TopicDataId, TopicDataValue};
use elf_runtime_model_kernel::{TopicDataProvider, TopicSchema, TopicService};
use std::sync::Arc;

pub struct PipelineExecuteTopicData {
    previous: Option<ArcTopicData>,
    current: Option<ArcTopicData>,
    internal_data_id: Arc<TopicDataId>,
}

impl PipelineExecuteTopicData {
    /// get topic data id from given topic data
    /// raise error if the value is not a string or a decimal.
    fn get_data_id(data: &TopicData) -> StdR<Arc<TopicDataId>> {
        let data_id = data.get(TopicDataColumnNames::Id.column_name());
        match data_id {
            Some(data_id) => match data_id {
                TopicDataValue::Str(data_id) => Ok(Arc::new(data_id.clone())),
                TopicDataValue::Num(data_id) => Ok(Arc::new(data_id.to_string())),
                _ => PipelineKernelErrorCode::TopicDataIdTypeNotSupported.msg(format!(
                    "Topic data id type not supported, of data[{:?}].",
                    data
                )),
            },
            _ => PipelineKernelErrorCode::TopicDataIdNotFound
                .msg(format!("Topic data id not found, of data[{:?}].", data)),
        }
    }

    /// save given data to topic data storage
    pub fn insert(data: TopicData, topic_schema: &Arc<TopicSchema>) -> StdR<Self> {
        let topic_data_service = TopicService::data()?;
        let current_data = topic_data_service.insert(topic_schema, data)?;
        let data_id = Self::get_data_id(&current_data)?;

        Ok(Self {
            current: Some(ArcTopicData::build(current_data)),
            previous: None,
            internal_data_id: data_id,
        })
    }

    /// given data is from synonym
    /// set default internal data id as [-1], since there is no way to set topic data id into a synonym table.
    pub fn insert_into_synonym(data: TopicData) -> StdR<Self> {
        Ok(Self {
            current: Some(ArcTopicData::build(data)),
            previous: None,
            internal_data_id: Arc::new("-1".to_string()),
        })
    }

    pub fn insert_or_merge(data: TopicData, topic_schema: &Arc<TopicSchema>) -> StdR<Self> {
        let topic_data_service = TopicService::data()?;
        let (previous_data, current_data) =
            topic_data_service.insert_or_merge(topic_schema, data)?;
        let data_id = Self::get_data_id(&current_data)?;

        match previous_data {
            Some(previous_data) => Ok(Self {
                current: Some(ArcTopicData::build(current_data)),
                previous: Some(ArcTopicData::build(previous_data)),
                internal_data_id: data_id,
            }),
            _ => Ok(Self {
                current: Some(ArcTopicData::build(current_data)),
                previous: None,
                internal_data_id: data_id,
            }),
        }
    }

    pub fn merge(data: TopicData, topic_schema: &Arc<TopicSchema>) -> StdR<Self> {
        let topic_data_service = TopicService::data()?;
        let (previous_data, current_data) = topic_data_service.merge(topic_schema, data)?;
        let data_id = Self::get_data_id(&current_data)?;

        Ok(Self {
            current: Some(ArcTopicData::build(current_data)),
            previous: Some(ArcTopicData::build(previous_data)),
            internal_data_id: data_id,
        })
    }

    pub fn delete(data: TopicData, topic_schema: &Arc<TopicSchema>) -> StdR<Self> {
        let topic_data_service = TopicService::data()?;
        let previous_data = topic_data_service.delete(topic_schema, data)?;
        let data_id = Self::get_data_id(&previous_data)?;

        Ok(Self {
            current: None,
            previous: Some(ArcTopicData::build(previous_data)),
            internal_data_id: data_id,
        })
    }
}

impl PipelineExecuteTopicData {
    pub fn previous_data(&self) -> &Option<ArcTopicData> {
        &self.previous
    }

    pub fn current_data(&self) -> &Option<ArcTopicData> {
        &self.current
    }

    pub fn topic_data_id(&self) -> &Arc<TopicDataId> {
        &self.internal_data_id
    }
}
