use crate::{BaseDataModel, FactorId, ParameterKind, Storable, TopicId};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TopicFactorParameter {
    pub kind: Option<ParameterKind>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}
