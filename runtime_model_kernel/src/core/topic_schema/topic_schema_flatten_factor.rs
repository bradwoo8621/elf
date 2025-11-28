use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner, TopicSchemaGroupFactor,
};
use std::sync::Arc;
use watchmen_model::TopicData;

#[derive(Debug)]
pub struct TopicSchemaFlattenFactor {
    inner: TopicSchemaFactorInner,
}

impl TopicSchemaFlattenFactor {
    pub fn new(inner: TopicSchemaFactorInner) -> Self {
        Self { inner }
    }
}

impl TopicSchemaFactor for TopicSchemaFlattenFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

impl TopicSchemaGroupFactor<TopicSchemaFlattenFactor> for TopicSchemaFlattenFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> TopicSchemaFlattenFactor {
        TopicSchemaFlattenFactor {
            inner: self.get_inner().replace_names(names),
        }
    }
}

pub type TopicSchemaFlattenFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaFlattenFactor, TopicSchemaFlattenFactorGroup>;

#[derive(Debug)]
pub struct TopicSchemaFlattenFactorGroup {
    inner: TopicSchemaFlattenFactorGroupInner,
}

impl TopicSchemaFlattenFactorGroup {
    pub fn new(inner: TopicSchemaFlattenFactorGroupInner) -> Self {
        Self { inner }
    }

    /// flatten the value from hierarchical object to root level with "dot connected field name"
    pub fn flatten(&self, _data: &mut TopicData) {
        todo!("implement flatten for TopicSchemaFlattenFactorGroup")
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaFlattenFactor, TopicSchemaFlattenFactorGroup>
    for TopicSchemaFlattenFactorGroup
{
    type Inner = TopicSchemaFlattenFactorGroupInner;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<TopicSchemaFlattenFactor>>>) -> Self {
        Self::new(TopicSchemaFactorGroupInner::new(name, factors))
    }

    fn get_inner(&self) -> &TopicSchemaFlattenFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaFlattenFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaFlattenFactor, TopicSchemaFlattenFactorGroup>
    for TopicSchemaFlattenFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaFlattenFactor {
        TopicSchemaFlattenFactor::new(TopicSchemaFactorInner::new(factor.clone()))
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaFlattenFactor>>>,
    ) -> TopicSchemaFlattenFactorGroup {
        TopicSchemaFlattenFactorGroup::new(TopicSchemaFlattenFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
