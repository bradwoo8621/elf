use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner, TopicSchemaGroupFactor,
};
use std::sync::Arc;
use watchmen_model::{FactorEncryptMethod, TopicData};

#[derive(Debug)]
pub struct TopicSchemaEncryptFactor {
    inner: TopicSchemaFactorInner,
}

impl TopicSchemaEncryptFactor {
    pub fn new(inner: TopicSchemaFactorInner) -> Self {
        Self { inner }
    }

    pub fn encrypt_method(&self) -> &Option<Arc<FactorEncryptMethod>> {
        &self.factor().encrypt
    }
}

impl TopicSchemaFactor for TopicSchemaEncryptFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

impl TopicSchemaGroupFactor<TopicSchemaEncryptFactor> for TopicSchemaEncryptFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> TopicSchemaEncryptFactor {
        TopicSchemaEncryptFactor {
            inner: self.get_inner().replace_names(names),
        }
    }
}

pub type TopicSchemaEncryptFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaEncryptFactor, TopicSchemaEncryptFactorGroup>;

#[derive(Debug)]
pub struct TopicSchemaEncryptFactorGroup {
    inner: TopicSchemaEncryptFactorGroupInner,
}

impl TopicSchemaEncryptFactorGroup {
    pub fn new(inner: TopicSchemaEncryptFactorGroupInner) -> Self {
        Self { inner }
    }

    pub fn encrypt(&self, _data: &mut TopicData) {
        todo!("implement encrypt for TopicSchemaEncryptFactorGroup")
    }

    pub fn decrypt(&self, _data: &mut TopicData) {
        todo!("implement decrypt for TopicSchemaEncryptFactorGroup")
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaEncryptFactor, TopicSchemaEncryptFactorGroup>
    for TopicSchemaEncryptFactorGroup
{
    type Inner = TopicSchemaEncryptFactorGroupInner;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<TopicSchemaEncryptFactor>>>) -> Self {
        Self::new(TopicSchemaFactorGroupInner::new(name, factors))
    }

    fn get_inner(&self) -> &TopicSchemaEncryptFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaEncryptFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaEncryptFactor, TopicSchemaEncryptFactorGroup>
    for TopicSchemaEncryptFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaEncryptFactor {
        TopicSchemaEncryptFactor::new(TopicSchemaFactorInner::new(factor.clone()))
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaEncryptFactor>>>,
    ) -> TopicSchemaEncryptFactorGroup {
        TopicSchemaEncryptFactorGroup::new(TopicSchemaEncryptFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
