use crate::{
	BaseDataModel, GraphicPosition, GraphicRect, Storable, TenantId, TopicId, UserBasedTuple,
	UserId,
};
use chrono::NaiveDateTime;
use elf_base::serde::option_naive_datetime;
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TopicRectInPipelineGraphic {
    pub coordinate: Option<GraphicPosition>,
    pub frame: Option<GraphicRect>,
    pub name: Option<GraphicPosition>,
}

#[adapt_model(storable)]
pub struct TopicInPipelineGraphic {
    pub topic_id: Option<TopicId>,
    pub rect: Option<TopicRectInPipelineGraphic>,
}

pub type PipelineGraphicId = String;

#[adapt_model(user_based)]
pub struct PipelineGraphic {
    pub pipeline_graph_id: Option<PipelineGraphicId>,
    pub name: Option<String>,
    pub topics: Option<Vec<TopicInPipelineGraphic>>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "option_naive_datetime")]
    pub last_modified_at: Option<NaiveDateTime>,
}
