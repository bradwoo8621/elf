use crate::TopicSchemaFactor;
use elf_base::DisplayLines;

pub struct TopicSchemaFactorUtils;

impl TopicSchemaFactorUtils {
    pub fn factors_to_str(factors: &Vec<TopicSchemaFactor>) -> String {
        factors
            .iter()
            .map(|f| format!("{}", f))
            .map(DisplayLines::indent)
            .collect::<Vec<String>>()
            .join(",\n")
    }

    pub fn filter_functional_factors(factors: Vec<TopicSchemaFactor>) -> Vec<TopicSchemaFactor> {
        factors
            .into_iter()
            .map(|f| f.if_functional())
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
            .collect()
    }
}
