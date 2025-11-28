use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner, TopicSchemaGroupFactor,
};
use std::sync::Arc;
use watchmen_model::TopicData;

#[derive(Debug)]
pub struct TopicSchemaDateOrTimeFactor {
    inner: TopicSchemaFactorInner,
}

impl TopicSchemaDateOrTimeFactor {
    pub fn new(inner: TopicSchemaFactorInner) -> Self {
        Self { inner }
    }
}

impl TopicSchemaFactor for TopicSchemaDateOrTimeFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

impl TopicSchemaGroupFactor<TopicSchemaDateOrTimeFactor> for TopicSchemaDateOrTimeFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> TopicSchemaDateOrTimeFactor {
        TopicSchemaDateOrTimeFactor {
            inner: self.get_inner().replace_names(names),
        }
    }
}

pub type TopicSchemaDateOrTimeFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaDateOrTimeFactor, TopicSchemaDateOrTimeFactorGroup>;

#[derive(Debug)]
pub struct TopicSchemaDateOrTimeFactorGroup {
    inner: TopicSchemaDateOrTimeFactorGroupInner,
}

impl TopicSchemaDateOrTimeFactorGroup {
    pub fn new(inner: TopicSchemaDateOrTimeFactorGroupInner) -> Self {
        Self { inner }
    }

    /// try to cast the value to date or time, if it is not
    pub fn try_cast_to_datetime(&self, _data: &mut TopicData) {
        todo!("implement encrypt for TopicSchemaDateOrTimeFactorGroup")
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaDateOrTimeFactor, TopicSchemaDateOrTimeFactorGroup>
    for TopicSchemaDateOrTimeFactorGroup
{
    type Inner = TopicSchemaDateOrTimeFactorGroupInner;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<TopicSchemaDateOrTimeFactor>>>) -> Self {
        Self::new(TopicSchemaFactorGroupInner::new(name, factors))
    }

    fn get_inner(&self) -> &TopicSchemaDateOrTimeFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaDateOrTimeFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaDateOrTimeFactor, TopicSchemaDateOrTimeFactorGroup>
    for TopicSchemaDateOrTimeFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaDateOrTimeFactor {
        TopicSchemaDateOrTimeFactor::new(TopicSchemaFactorInner::new(factor.clone()))
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaDateOrTimeFactor>>>,
    ) -> TopicSchemaDateOrTimeFactorGroup {
        TopicSchemaDateOrTimeFactorGroup::new(TopicSchemaDateOrTimeFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
