use crate::PipelineExecutionVariables;
use watchmen_model::{StdR, TopicDataValue};

pub trait InMemoryParameter {
    fn value_from<'a>(&self, variables: &'a PipelineExecutionVariables)
    -> StdR<&'a TopicDataValue>;
}
