use crate::{
	BaseDataModel, ConnectedSpaceId, GraphicRect, Storable, TenantId, TopicId, UserBasedTuple,
	UserId,
};
use elf_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TopicOfConnectedSpaceGraphic {
    pub topic_id: Option<TopicId>,
    pub rect: Option<GraphicRect>,
}

#[adapt_model(storable)]
pub struct SubjectOfConnectedSpaceGraphic {
    pub subject_d: Option<TopicId>,
    pub rect: Option<GraphicRect>,
}

#[adapt_model(user_based)]
pub struct ConnectedSpaceGraphic {
    pub connect_d: Option<ConnectedSpaceId>,
    pub topics: Option<Vec<TopicOfConnectedSpaceGraphic>>,
    pub subjects: Option<Vec<SubjectOfConnectedSpaceGraphic>>,
}
