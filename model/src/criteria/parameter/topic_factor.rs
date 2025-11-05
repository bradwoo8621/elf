use crate::BaseDataModel;
use crate::FactorId;
use crate::TopicId;

pub struct TopicFactorParameter {
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

impl BaseDataModel for TopicFactorParameter {}
