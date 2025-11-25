use crate::{ArcFactor, ArcTopic, TopicSchemaFactor};
use std::collections::HashMap;
use std::sync::Arc;

pub trait TopicSchemaFactorGroups<F, G> {
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool;

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> F;

    fn create_schema_group(name: String, factors: Arc<Vec<Arc<F>>>) -> G;

    fn create(topic: &Arc<ArcTopic>) -> Option<Arc<Vec<Arc<G>>>>
    where
        F: TopicSchemaFactor,
    {
        let factors = &topic.factors;
        if factors.is_none() {
            return None;
        }

        let factors = factors.as_ref().unwrap();
        if factors.is_empty() {
            return None;
        }

        let groups = factors
            .as_ref()
            .into_iter()
            // check the given factor is accepted or not
            .filter(|factor| Self::accept_factor(factor))
            // map to schema factor
            .map(|factor| Self::create_schema_factor(&factor))
            // group by the first name segment
            .fold(HashMap::<String, Vec<Arc<F>>>::new(), |mut acc, item| {
                let name0 = &item.names()[0];
                acc.entry(name0.clone())
                    .or_insert_with(Vec::new)
                    .push(Arc::new(item));
                acc
            })
            .into_iter()
            // map to schema factor group
            .map(|(name, factors)| Self::create_schema_group(name, Arc::new(factors)))
            // collect to vec
            .map(Arc::new)
            .collect();

        Some(Arc::new(groups))
    }
}
