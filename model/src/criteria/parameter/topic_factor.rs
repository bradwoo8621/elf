use crate::{BaseDataModel, FactorId, Parameter, ParameterKind, Storable, TopicId};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TopicFactorParameter {
    pub kind: Option<ParameterKind>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

impl TopicFactorParameter {
    pub fn init() -> Self {
        Self::new().kind(ParameterKind::Topic)
    }

    pub fn of(topic_id: TopicId, factor_id: FactorId) -> Self {
        Self::init()
            .topic_id(topic_id)
            .factor_id(factor_id)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Topic(self)
    }
}
