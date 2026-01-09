use crate::ArcHelper;
use elf_base::StdR;
use elf_model::{FactorId, ParameterKind, TopicFactorParameter, TopicId};
use std::sync::Arc;

#[derive(Debug)]
pub struct ArcTopicFactorParameter {
    pub kind: Arc<ParameterKind>,
    pub topic_id: Arc<TopicId>,
    pub factor_id: Arc<FactorId>,
}

impl ArcHelper for ArcTopicFactorParameter {}

impl ArcTopicFactorParameter {
    pub fn new(parameter: TopicFactorParameter) -> StdR<Arc<Self>> {
        let topic_id = Self::topic_id(parameter.topic_id, || "Topic factor parameter")?;
        let factor_id = Self::factor_id(parameter.factor_id, || "Topic factor parameter")?;

        Ok(Arc::new(Self {
            kind: Arc::new(ParameterKind::Topic),
            topic_id,
            factor_id,
        }))
    }
}
