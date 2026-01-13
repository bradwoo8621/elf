use crate::{
    ArcFactor, ArcTopic, FakeTopicSchemaFactor, RuntimeModelKernelErrorCode,
    SimpleTopicSchemaFactor, TopicSchemaFactor, TopicSchemaFactorUtils, VecOrMapTopicSchemaFactor,
};
use elf_base::{ErrorCode, StdR, VoidR};
use elf_model::FactorType;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

struct TopicSchemaFactorsContext {
    topic: Arc<ArcTopic>,
    factors: HashMap<String, Arc<ArcFactor>>,
    sorted_factor_names: Vec<String>,

    top_level_names: BTreeSet<String>,
    schema_factors: HashMap<String, TopicSchemaFactor>,
    /// hierarchy of factors
    /// for example:
    /// - a -> [a.b, a.c],
    /// - a.b -> [a.b.d],
    /// - a.c -> [a.c.e, a.c.f]
    hierarchy: HashMap<String, BTreeSet<String>>,
}

impl TopicSchemaFactorsContext {
    fn new(topic: &Arc<ArcTopic>) -> Self {
        // create a factor map, easy to locate factor by full qualified factor name
        let (factor_map, mut sorted_factor_names) = topic.factors.iter().fold(
            (HashMap::new(), vec![]),
            |mut acc: (HashMap<_, _>, Vec<_>), factor| {
                acc.0.insert(factor.name.deref().clone(), factor.clone());
                acc.1.push(factor.name.deref().clone());
                acc
            },
        );
        sorted_factor_names.sort_by(|n1, n2| n1.cmp(n2));
        Self {
            topic: topic.clone(),
            factors: factor_map,
            sorted_factor_names,

            top_level_names: BTreeSet::new(),
            schema_factors: HashMap::new(),
            hierarchy: HashMap::new(),
        }
    }
}

struct TopicSchemaFactorsBuilder;

impl TopicSchemaFactorsBuilder {
    fn create_schema_factor(
        factor: Option<Arc<ArcFactor>>,
        name: &String,
        full_name: &String,
    ) -> TopicSchemaFactor {
        match factor {
            Some(factor) => match factor.r#type.deref() {
                FactorType::Object | FactorType::Array => TopicSchemaFactor::VecOrMap(
                    VecOrMapTopicSchemaFactor::new(factor.clone(), name.clone()),
                ),
                _ => TopicSchemaFactor::Simple(SimpleTopicSchemaFactor::new(
                    factor.clone(),
                    name.clone(),
                )),
            },
            _ => {
                TopicSchemaFactor::Fake(FakeTopicSchemaFactor::new(full_name.clone(), name.clone()))
            }
        }
    }

    fn find_factor_by_name(
        factor_name: &String,
        context: &mut TopicSchemaFactorsContext,
    ) -> StdR<Arc<ArcFactor>> {
        if let Some(found) = context.factors.get(factor_name) {
            Ok(found.clone())
        } else {
            RuntimeModelKernelErrorCode::TopicSchemaGenerate
                .msg(format!("Factor not found by name[{}].", factor_name))
        }
    }

    fn collect_factor(
        factor: Option<Arc<ArcFactor>>,
        names: &Vec<String>,
        end_exclusive: usize,
        context: &mut TopicSchemaFactorsContext,
    ) -> VoidR {
        match end_exclusive {
            // only occurs when no names split by factor name, which never happens
            0 => RuntimeModelKernelErrorCode::TopicSchemaGenerate.msg(format!(
                "Incorrect factor name[topic_id={}, factor_id={}, factor_name={}].",
                context.topic.topic_id,
                factor
                    .as_ref()
                    .map(|f| f.factor_id.deref().as_str())
                    .unwrap_or(""),
                factor
                    .as_ref()
                    .map(|f| f.name.deref().as_str())
                    .unwrap_or(""),
            )),
            1 => {
                // the first part of names (names might have only one part)
                // no need to attach to parent, it is top level
                let name = &names[0];
                let schema_factor = Self::create_schema_factor(factor, name, name);
                // collect to schema factors
                context.schema_factors.insert(name.clone(), schema_factor);
                // collect to top level names
                context.top_level_names.insert(name.clone());
                // collect to hierarchy
                context
                    .hierarchy
                    .entry(name.clone())
                    .or_insert_with(BTreeSet::new);
                Ok(())
            }
            _ => {
                // not the first part of name
                // handle parent first
                let parent_factor_name = names[0..end_exclusive - 1].join(".");
                let parent_factor = Self::find_factor_by_name(&parent_factor_name, context).ok();
                Self::collect_factor(parent_factor, names, end_exclusive - 1, context)?;
                // and itself
                let name = &names[end_exclusive - 1];
                let factor_name = names[0..end_exclusive].join(".");
                let schema_factor = Self::create_schema_factor(factor, name, &factor_name);
                // collect to schema factors
                context
                    .schema_factors
                    .insert(factor_name.clone(), schema_factor);
                if let Some(children_of_parent) = context.hierarchy.get_mut(&parent_factor_name) {
                    // collect to hierarchy, as child of parent
                    children_of_parent.insert(factor_name.clone());
                    // collect to hierarchy
                    context
                        .hierarchy
                        .entry(factor_name)
                        .or_insert_with(BTreeSet::new);
                    Ok(())
                } else {
                    RuntimeModelKernelErrorCode::TopicSchemaGenerate.msg(format!(
                        "Parent factor[topic_id={}, factor_name={}] not found.",
                        context.topic.topic_id, parent_factor_name
                    ))
                }
            }
        }
    }

    fn collect_by_factor_name(
        factor_name: &String,
        context: &mut TopicSchemaFactorsContext,
    ) -> VoidR {
        let factor = Self::find_factor_by_name(factor_name, context)?;
        let names: Vec<String> = factor_name.split(".").map(String::from).collect();
        Self::collect_factor(Some(factor), &names, names.len(), context)
    }

    fn collect_factors(context: &mut TopicSchemaFactorsContext) -> VoidR {
        for factor_name in context.sorted_factor_names.clone() {
            Self::collect_by_factor_name(&factor_name, context)?;
        }

        Ok(())
    }

    fn build_factor_hierarchy(
        factor: &mut TopicSchemaFactor,
        factor_name: &String,
        hierarchy: &HashMap<String, BTreeSet<String>>,
        schema_factors: &mut HashMap<String, TopicSchemaFactor>,
    ) -> VoidR {
        if let Some(child_names) = hierarchy.get(factor_name) {
            for child_name in child_names {
                if let Some(mut child_factor) = schema_factors.remove(child_name) {
                    match factor {
                        TopicSchemaFactor::Simple(_) => {
                            return RuntimeModelKernelErrorCode::TopicSchemaGenerate
                                .msg(format!("Parent factor[{}] cannot be simple.", factor_name));
                        }
                        TopicSchemaFactor::VecOrMap(vec_or_map) => {
                            Self::build_factor_hierarchy(
                                &mut child_factor,
                                &child_name,
                                hierarchy,
                                schema_factors,
                            )?;
                            vec_or_map.children.push(child_factor);
                        }
                        TopicSchemaFactor::Fake(fake) => {
                            Self::build_factor_hierarchy(
                                &mut child_factor,
                                &child_name,
                                hierarchy,
                                schema_factors,
                            )?;
                            fake.children.push(child_factor);
                        }
                    }
                } else {
                    return RuntimeModelKernelErrorCode::TopicSchemaGenerate
                        .msg(format!("Factor not found by name[{}].", child_name));
                }
            }
        }

        Ok(())
    }

    fn create_factors(context: &mut TopicSchemaFactorsContext) -> StdR<TopicSchemaFactors> {
        let mut factors = vec![];
        for top_level_name in context.top_level_names.iter() {
            if let Some(mut factor) = context.schema_factors.remove(top_level_name) {
                Self::build_factor_hierarchy(
                    &mut factor,
                    &top_level_name,
                    &context.hierarchy,
                    &mut context.schema_factors,
                )?;
                factors.push(factor);
            } else {
                return RuntimeModelKernelErrorCode::TopicSchemaGenerate
                    .msg(format!("Factor not found by name[{}].", top_level_name));
            }
        }

        Ok(TopicSchemaFactors { factors })
    }
}

pub struct TopicSchemaFactors {
    pub factors: Vec<TopicSchemaFactor>,
}

impl Display for TopicSchemaFactors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.factors.is_empty() {
            write!(f, "TopicSchemaFactors[factors=[]]")
        } else {
            write!(
                f,
                "TopicSchemaFactors[factors=[\n{}\n]]",
                TopicSchemaFactorUtils::factors_to_str(&self.factors)
            )
        }
    }
}

impl TopicSchemaFactors {
    pub fn of_topic(topic: &Arc<ArcTopic>) -> StdR<Self> {
        let mut context = TopicSchemaFactorsContext::new(topic);
        TopicSchemaFactorsBuilder::collect_factors(&mut context)?;
        TopicSchemaFactorsBuilder::create_factors(&mut context)
    }

    pub fn if_functional(self) -> Option<Arc<Self>> {
        let functional_factors: Vec<TopicSchemaFactor> =
            TopicSchemaFactorUtils::filter_functional_factors(self.factors);

        if functional_factors.is_empty() {
            None
        } else {
            Some(Arc::new(Self {
                factors: functional_factors,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArcTopic, TopicSchemaFactors};
    use elf_model::{Factor, FactorType, Topic, TopicKind, TopicType};

    #[test]
    fn test_split() {
        let parts: Vec<String> = "".split('.').map(String::from).collect();
        assert_eq!(parts.len(), 1);

        assert_eq!(vec!["abc"][0..1].join("."), "abc");
    }

    fn create_topic() -> Topic {
        Topic::new()
            .topic_id("1".to_string())
            .name("test-topic".to_string())
            .r#type(TopicType::Distinct)
            .kind(TopicKind::Business)
            .factors(vec![
                Factor::new()
                    .factor_id("101".to_string())
                    .name("name".to_string())
                    .r#type(FactorType::Text),
                Factor::new()
                    .factor_id("102".to_string())
                    .name("age".to_string())
                    .r#type(FactorType::Number),
                Factor::new()
                    .factor_id("103".to_string())
                    .name("addresses".to_string())
                    .r#type(FactorType::Array),
                Factor::new()
                    .factor_id("104".to_string())
                    .name("addresses.city".to_string())
                    .r#type(FactorType::Text),
                Factor::new()
                    .factor_id("105".to_string())
                    .name("work".to_string())
                    .r#type(FactorType::Object),
                Factor::new()
                    .factor_id("106".to_string())
                    .name("work.addresses".to_string())
                    .r#type(FactorType::Array),
                Factor::new()
                    .factor_id("107".to_string())
                    .name("work.addresses.city".to_string())
                    .r#type(FactorType::Text),
                Factor::new()
                    .factor_id("108".to_string())
                    .name("work.unknown.name".to_string())
                    .r#type(FactorType::Text),
            ])
            .tenant_id("1".to_string())
            .build()
    }

    #[test]
    fn test_topic() {
        let topic = ArcTopic::new(create_topic()).expect("Failed to create topic.");
        let schema_factors =
            TopicSchemaFactors::of_topic(&topic).expect("Failed to create schema factors.");
        println!("{}", schema_factors)
    }
}
