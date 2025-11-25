use crate::TopicSchemaFactor;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TopicSchemaFactorGroupInner<F, G> {
    name: Arc<String>,
    factors: Option<Arc<Vec<Arc<F>>>>,
    groups: Option<Arc<HashMap<String, Arc<G>>>>,
}

impl<F, G> TopicSchemaFactorGroupInner<F, G> {
    pub fn new(name: Arc<String>, factors: Arc<Vec<Arc<F>>>) -> Self
    where
        F: TopicSchemaFactor,
    {
        let (factors, groups) = if factors.is_empty() {
            (None, None)
        } else {
            TopicSchemaFactorGroupInner::split_factors(factors)
        };

        TopicSchemaFactorGroupInner {
            name,
            factors,
            groups,
        }
    }

    fn split_factors(
        factors: Arc<Vec<Arc<F>>>,
    ) -> (
        Option<Arc<Vec<Arc<F>>>>,
        Option<Arc<HashMap<String, Arc<G>>>>,
    )
    where
        F: TopicSchemaFactor,
    {
        let (factors, groups) =
            factors
                .as_ref()
                .into_iter()
                .fold((Vec::new(), HashMap::new()), |mut acc, factor| {
                    let names = factor.names();
                    if names.len() == 1 {
                        acc.0.push(factor.clone());
                    } else {
                    }
                    acc
                });

        (
            if factors.is_empty() {
                None
            } else {
                Some(Arc::new(factors))
            },
            if groups.is_empty() {
                None
            } else {
                Some(Arc::new(groups))
            },
        )
    }
}

pub trait TopicSchemaFactorGroupInnerOp<F, G> {
    fn name(&self) -> &Arc<String>;
    fn factors(&self) -> &Option<Arc<Vec<Arc<F>>>>;
    fn groups(&self) -> &Option<Arc<HashMap<String, Arc<G>>>>;
}

impl<F, G> TopicSchemaFactorGroupInnerOp<F, G> for TopicSchemaFactorGroupInner<F, G> {
    fn name(&self) -> &Arc<String> {
        &self.name
    }

    fn factors(&self) -> &Option<Arc<Vec<Arc<F>>>> {
        &self.factors
    }

    fn groups(&self) -> &Option<Arc<HashMap<String, Arc<G>>>> {
        &self.groups
    }
}

pub trait TopicSchemaFactorGroup<'a, F, G> {
    type Inner: TopicSchemaFactorGroupInnerOp<F, G> + 'a;

    fn get_inner(&self) -> &Self::Inner;

    fn name(&'a self) -> &'a Arc<String> {
        &self.get_inner().name()
    }

    fn factors(&'a self) -> &'a Option<Arc<Vec<Arc<F>>>> {
        &self.get_inner().factors()
    }

    fn groups(&'a self) -> &'a Option<Arc<HashMap<String, Arc<G>>>> {
        &self.get_inner().groups()
    }
}
