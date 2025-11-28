use crate::{TopicSchemaFactor, TopicSchemaGroupFactor};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct TopicSchemaFactorGroupInner<F, G> {
    name: Arc<String>,
    factors: Option<Arc<Vec<Arc<F>>>>,
    groups: Option<Arc<Vec<Arc<G>>>>,
}

impl<F, G> TopicSchemaFactorGroupInner<F, G> {
    pub fn new<'a>(name: Arc<String>, factors: Arc<Vec<Arc<F>>>) -> Self
    where
        F: TopicSchemaFactor + TopicSchemaGroupFactor<F>,
        G: TopicSchemaFactorGroup<'a, F, G>,
    {
        let (factors, groups) = if factors.is_empty() {
            // not happened in practice
            (None, None)
        } else {
            Self::split_factors(factors)
        };

        Self {
            name,
            factors,
            groups,
        }
    }

    fn split_factors<'a>(
        factors: Arc<Vec<Arc<F>>>,
    ) -> (Option<Arc<Vec<Arc<F>>>>, Option<Arc<Vec<Arc<G>>>>)
    where
        F: TopicSchemaFactor + TopicSchemaGroupFactor<F>,
        G: TopicSchemaFactorGroup<'a, F, G>,
    {
        let (factors, groups) =
            factors
                .as_ref()
                .into_iter()
                .fold((Vec::new(), HashMap::new()), |mut acc, factor| {
                    let names = factor.names();
                    if names.len() == 1 {
                        // single name
                        acc.0.push(factor.clone());
                    } else {
                        // multiple names, treat as group
                        let group_name = names[0].clone();
                        let new_factor = factor.remove_first_name();
                        acc.1
                            .entry(group_name)
                            .or_insert_with(Vec::new)
                            .push(Arc::new(new_factor));
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
                let groups_vec = groups
                    .into_iter()
                    .map(|(name, factors)| G::new(Arc::new(name), Arc::new(factors)))
                    .map(Arc::new)
                    .collect();
                Some(Arc::new(groups_vec))
            },
        )
    }
}

pub trait TopicSchemaFactorGroupInnerOp<F, G> {
    fn name(&self) -> &Arc<String>;
    fn factors(&self) -> &Option<Arc<Vec<Arc<F>>>>;
    fn groups(&self) -> &Option<Arc<Vec<Arc<G>>>>;
}

impl<F, G> TopicSchemaFactorGroupInnerOp<F, G> for TopicSchemaFactorGroupInner<F, G> {
    fn name(&self) -> &Arc<String> {
        &self.name
    }

    fn factors(&self) -> &Option<Arc<Vec<Arc<F>>>> {
        &self.factors
    }

    fn groups(&self) -> &Option<Arc<Vec<Arc<G>>>> {
        &self.groups
    }
}

pub trait TopicSchemaFactorGroup<'a, F, G> {
    type Inner: TopicSchemaFactorGroupInnerOp<F, G> + 'a;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<F>>>) -> Self;

    fn get_inner(&self) -> &Self::Inner;

    fn name(&'a self) -> &'a Arc<String> {
        &self.get_inner().name()
    }

    fn factors(&'a self) -> &'a Option<Arc<Vec<Arc<F>>>> {
        &self.get_inner().factors()
    }

    fn groups(&'a self) -> &'a Option<Arc<Vec<Arc<G>>>> {
        &self.get_inner().groups()
    }
}
