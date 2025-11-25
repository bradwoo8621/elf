use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner,
};
use std::sync::Arc;
use watchmen_model::TopicDataValue;

pub struct TopicSchemaDefaultValueFactor {
    inner: TopicSchemaFactorInner,
    default_value: Option<TopicDataValue>,
}

impl TopicSchemaDefaultValueFactor {
    pub fn new(inner: TopicSchemaFactorInner, default_value: Option<TopicDataValue>) -> Self {
        TopicSchemaDefaultValueFactor {
            inner,
            default_value,
        }
    }

    pub fn default_value(&self) -> &Option<TopicDataValue> {
        &self.default_value
    }
}

impl TopicSchemaFactor for TopicSchemaDefaultValueFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

pub type TopicSchemaDefaultValueFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>;

pub struct TopicSchemaDefaultValueFactorGroup {
    inner: TopicSchemaDefaultValueFactorGroupInner,
}

impl TopicSchemaDefaultValueFactorGroup {
    pub fn new(inner: TopicSchemaDefaultValueFactorGroupInner) -> Self {
        TopicSchemaDefaultValueFactorGroup { inner }
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroup
{
    type Inner = TopicSchemaDefaultValueFactorGroupInner;

    fn get_inner(&self) -> &TopicSchemaDefaultValueFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaDefaultValueFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaDefaultValueFactor {
        TopicSchemaDefaultValueFactor {
            inner: TopicSchemaFactorInner::new(factor.clone()),
            default_value: None,
        }
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaDefaultValueFactor>>>,
    ) -> TopicSchemaDefaultValueFactorGroup {
        TopicSchemaDefaultValueFactorGroup::new(TopicSchemaDefaultValueFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
