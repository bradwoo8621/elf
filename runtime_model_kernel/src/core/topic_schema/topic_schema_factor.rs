use crate::ArcFactor;
use std::sync::Arc;

#[derive(Debug)]
pub struct TopicSchemaFactorInner {
    factor: Arc<ArcFactor>,
    factor_name: Arc<String>,
    names: Arc<Vec<String>>,
}

impl TopicSchemaFactorInner {
    pub fn new(factor: Arc<ArcFactor>) -> Self {
        let factor_name = factor.name.clone();
        let names = Arc::new(factor_name.split('.').map(String::from).collect());

        Self {
            factor,
            factor_name,
            names,
        }
    }

    pub fn replace_names(&self, names: Arc<Vec<String>>) -> Self {
        Self {
            factor: self.factor.clone(),
            factor_name: self.factor_name.clone(),
            names,
        }
    }
}

pub trait TopicSchemaFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner;

    fn factor(&self) -> &Arc<ArcFactor> {
        &self.get_inner().factor
    }

    fn factor_name(&self) -> &Arc<String> {
        &self.get_inner().factor_name
    }

    fn names(&self) -> &Arc<Vec<String>> {
        &self.get_inner().names
    }
}

pub trait TopicSchemaGroupFactor<F>: TopicSchemaFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> F;

    fn remove_first_name(&self) -> F {
        let inner = self.get_inner();
        let mut names = inner.names.as_ref().clone();
        names.remove(0);
        self.replace_names(Arc::new(names))
    }
}
