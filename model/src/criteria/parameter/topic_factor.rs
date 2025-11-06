use crate::{BaseDataModel, FactorId, TopicId};
use watchmen_model_marco::adapt_model;

#[adapt_model(bdm)]
pub struct TopicFactorParameter {
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}
